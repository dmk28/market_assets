enum AssetType {
    Stocks(Stocks),
    Bonds(Bonds),
    RealEstate(RealEstate)
}

struct Stocks {
    value: f32,
    fluctuation: f32
}

impl Stocks {
    fn price(&self) -> f32 {
        self.value * (1.0 + self.fluctuation)
    }
}

struct Bonds {
    value: f32,
    fluctuation: f32
}

impl Bonds {
    fn price(&self) -> f32 {
        self.value * (1.0 + self.fluctuation)
    }
}

struct RealEstate {
    value: f32,
    fluctuation: f32,
    rating: f32
}

impl RealEstate {
    fn price(&self) -> f32 {
        self.value * (1.0 + self.fluctuation) * (1.0 - self.rating)
    }
}

fn main() {
    let assets = [
        AssetType::Stocks(Stocks{value: 15000.00, fluctuation: 0.015}),
        AssetType::Bonds(Bonds{value: 1300000.00, fluctuation: -0.25}),
        AssetType::RealEstate(RealEstate{value: 1000000.00, fluctuation: -0.10, rating: 0.50})
    ];

    let portfolio_value: f32 = assets.iter().map(|asset| match asset  {
        AssetType::Stocks(s) => s.price(),
        AssetType::Bonds(b) => b.price(),
        AssetType::RealEstate(r) => r.price(),
    }).sum();

    println!("Total value: {}", portfolio_value);
}