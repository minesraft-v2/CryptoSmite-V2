// Ensure you have added these dependencies to your Cargo.toml:
// [dependencies]
// headless_chrome = "1.15.0"
// anyhow = "1.0"

use headless_chrome::{Browser, LaunchOptions};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 1. Configure the browser launch options
    let options = LaunchOptions::default_builder()
        .headless(true) // Runs without opening a physical GUI window
        .build()?;

    // 2. Start the browser process and open a new tab
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    println!("Successfully connected to Chrome via DevTools Protocol.");

    // 3. Navigate to a safe target page
    tab.navigate_to("https://www.google.com")?;
    tab.wait_until_navigated()?;

    // 4. Inject and execute a safe script inside the page context
    // This evaluates a math calculation directly inside the browser's JS engine
    let script_to_run = "(() => { return 50 + 50; })()";
    let evaluation_result = tab.evaluate(script_to_run, false)?;

    // 5. Parse and print the return value from the browser context
    if let Some(value) = evaluation_result.value {
        println!("The script evaluated inside the browser returned: {}", value);
    } else {
        println!("The script executed but returned no value.");
    }

    Ok(())
    
