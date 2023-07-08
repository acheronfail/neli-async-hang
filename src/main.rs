use neli::{
    consts::{
        nl::{GenlId, NlmF},
        socket::NlFamily,
    },
    err::RouterError,
    genl::{AttrTypeBuilder, Genlmsghdr, GenlmsghdrBuilder, NlattrBuilder},
    nl::{NlPayload, Nlmsghdr},
    router::asynchronous::NlRouter,
    types::GenlBuffer,
    utils::Groups,
};

type Nl80211Payload = Genlmsghdr<u8, u16>;
type NextNl80211 =
    Option<Result<Nlmsghdr<GenlId, Nl80211Payload>, RouterError<GenlId, Nl80211Payload>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get the interface index from arguments
    // NOTE: change this to the index of your wifi card - use `ip addr` to get it
    let interface_idx = std::env::args()
        .nth(1)
        .expect("please pass wireless interface index!")
        .parse::<i32>()
        .expect("wireless interface index should be an integer");

    // connect to netlink
    let (socket, _) = NlRouter::connect(NlFamily::Generic, Some(0), Groups::empty()).await?;

    // get nl80211 family id
    let nl80211_family_id = socket.resolve_genl_family("nl80211").await?;

    // construct generic netlink attributes
    let mut genl_attrs = GenlBuffer::new();

    // IFINDEX is needed when requesting the GET_WIPHY command - which interface are we checking?
    genl_attrs.push(
        NlattrBuilder::default()
            // NOTE: 3 is the `IFINDEX` nl80211 attribute
            .nla_type(AttrTypeBuilder::default().nla_type(3).build()?)
            .nla_payload(interface_idx)
            .build()?,
    );

    // construct generic netlink message
    let genl_payload: Nl80211Payload = GenlmsghdrBuilder::default()
        .version(1)
        // NOTE: 1 is the `GET_WIPHY` nl80211 command
        .cmd(1)
        .attrs(genl_attrs)
        .build()?;

    // send it to netlink
    let mut recv = socket
        .send::<_, _, u16, Nl80211Payload>(
            nl80211_family_id,
            NlmF::REQUEST,
            NlPayload::Payload(genl_payload),
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
