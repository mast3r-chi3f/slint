slint::include_modules!();


const TAXPER    :f64 = 0.30;
const OWNERPER  :f64 = 0.55;
const PROFITPER :f64 = 0.05;
const OPEXPER   :f64 = 0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_handle = ui.as_weak();

    ui.on_devide_income({
        move |string| {
            let ui = ui_handle.unwrap();
            let income:f64 = string.trim().parse().unwrap();

            let tax         :f64 = income*TAXPER;
            let owner       :f64 = income*OWNERPER;
            let profit      :f64 = income*PROFITPER;
            let operation   :f64 = income*OPEXPER;

            let result      :String = format!("
Taxes   :{:.2}\n
Owner   :{:.2}\n
Profit  :{:.2}\n
OpEx    :{:.2}
            ", 
            tax,
            owner,
            profit,
            operation);

            ui.set_results(result.into());
        }
    });

    ui.run()
}
