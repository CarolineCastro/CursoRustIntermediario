//!A pumpkin is a cultivar of winter squash that is round with smooth, slightly ribbed skin, and is most often deep yellow to orange in coloration. 
//! The thick shell contains the seeds and pulp. The name is most commonly used for cultivars of Cucurbita pepo, but some cultivars of Cucurbita maxima, C. 
//! argyrosperma, and C. moschata with similar appearance are also sometimes called "pumpkins".
//! 
//! ![pumpkin image](https://upload.wikimedia.org/wikipedia/commons/thumb/5/5c/FrenchMarketPumpkinsB.jpg/700px-FrenchMarketPumpkinsB.jpg)

/// Big orange thing
/// 
/// #Recipes
/// 
/// Recipes will most assuredly be coming very, very soon!
pub struct Pumpkin {
    ///´roundness´ is a percentage
    pub roundness: f32,
    ///´orangeness´ is a number from 8 to 27
    pub orangeness: i32,
}

impl Pumpkin {
    /// If you smash the pumpkin, it will be gone. Then it can't be used for pie.
    pub fn smash(self) {}
}


///'BURT_ORANGE' is for the 'orangeness' field in the [Pumpkin] struct.
pub const BURNT_ORANGE: i32 = 13;

#[allow(dead_code)] // to silence the warning
enum PrivateEnum {
    /// For Halloween. To be lit by candlelight.
    JackOLantern,
    /// For dessert during North American winter holidays.
    PumpkinPie,
}