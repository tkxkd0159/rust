use bindings::{
    Windows::Foundation::Uri,
    Windows::Web::Syndication::SyndicationClient,
    Windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK},
};

fn main() -> windows::Result<()>{
    let uri = Uri::CreateUri("https://kennykerr.ca/feed")?;
    let client = SyndicationClient::new()?;
    let feed = client.RetrieveFeedAsync(uri)?.get()?;

    for item in feed.Items()? {
        println!("{}", item.Title()?.Text()?);
    }

    unsafe {
        MessageBoxA(None, "My Content", "My Title", MB_OK);
    }

    Ok(())
}
