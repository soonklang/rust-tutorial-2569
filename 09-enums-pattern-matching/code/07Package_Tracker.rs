fn main() {
    println!("====================== Package Status Tracker ======================");
    let status = vec![
        PackageStatus::Ordered,
        PackageStatus::Shipped("Kerry Express".to_string()), //ต้องแปลงเพราะที่รับมาคือ &str แต่ส่งไปหา String ใข้ String::from("Kerry Express")ได้
        PackageStatus::OutForDelivery{
            courier_name: String::from("Peewara"),
            estimated_hours: 1
        },
        PackageStatus::Delivered,
    ];
    for s in &status {
        println!("{}",describe_status(s));
    }
    println!("============================ Thank you ==============================");
}
enum PackageStatus{
    Ordered, // unit-like
    Shipped(String),//triple-like
    OutForDelivery{//strcut-like
        courier_name:String,
        estimated_hours :u32
    },
    Delivered,// unit-like
    
}
fn describe_status(status: &PackageStatus) -> String {
    match status{
        PackageStatus::Ordered => format!("Your order has been placed."),
        PackageStatus::Shipped(courier) => format!("Your order has been shipped via {}.",courier),
        PackageStatus::OutForDelivery{courier_name , estimated_hours } => format!("{} is delivering your package, arriving in about {} hour(s).",courier_name,estimated_hours ),
        PackageStatus::Delivered=> format!("Your package has been delivered!"),
    }

}