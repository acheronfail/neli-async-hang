use neli::{
    consts::{
        nl::NlmF,
        rtnl::{Arphrd, RtAddrFamily, Rtm},
        socket::NlFamily,
    },
    err::RouterError,
    nl::{NlPayload, Nlmsghdr},
    router::asynchronous::NlRouter,
    rtnl::{Ifinfomsg, IfinfomsgBuilder},
    utils::Groups,
};

type NextNl80211 = Option<Result<Nlmsghdr<Rtm, Ifinfomsg>, RouterError<Rtm, Ifinfomsg>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (socket, _) = NlRouter::connect(NlFamily::Route, None, Groups::empty()).await?;

    // needed from netlink route
    socket.enable_strict_checking(true)?;

    // build a message to fetch all interfaces
    let ifinfomsg = IfinfomsgBuilder::default()
        // this is layer 2, so family is unspecified
        .ifi_family(RtAddrFamily::Unspecified)
        .ifi_type(Arphrd::Netrom)
        // when index is zero, it fetches them all
        .ifi_index(0)
        .build()?;

    // send it to netlink
    let mut recv = socket
        .send::<Rtm, Ifinfomsg, Rtm, Ifinfomsg>(
            Rtm::Getlink,
            // NOTE: sending the `NlmF::DUMP` flag here will make this all work, but this example is testing the error case
            NlmF::REQUEST | NlmF::ACK,
            NlPayload::Payload(ifinfomsg),
        )
        .await?;

    // FIXME:
    // !!!!!!!!!!!!!!! after the first `Err(RouterError::Nlmsgerr(_))` message, the next call of `recv.next().await`
    // !!!!!!!!!!!!!!! blocks forever and hangs the tokio event loop
    while let Some(msg) = recv.next().await as NextNl80211 {
        let _ = dbg!(msg);
    }

    // FIXME: if there was an error above, then this is never logged, because `recv.next().await` hangs forever after the error
    eprintln!("finished iterating! if you're reading this then it worked! 🎉🎉🎉");

    Ok(())
}
