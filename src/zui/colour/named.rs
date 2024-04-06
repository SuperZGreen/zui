#![cfg(feature = "named_colours")]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(clippy::excessive_precision)]

use super::Colour;

// pub const 100Mph: Colour = Colour {r: 0.788235294117647f32, g: 0.24705882352941178f32, b: 0.2196078431372549f32, a: 1f32};
// pub const 20000LeaguesUnderTheSea: Colour = Colour {r: 0.09803921568627451f32, g: 0.09803921568627451f32, b: 0.4392156862745098f32, a: 1f32};
// pub const 24Carrot: Colour = Colour {r: 0.8980392156862745f32, g: 0.43137254901960786f32, b: 0.1411764705882353f32, a: 1f32};
// pub const 24Karat: Colour = Colour {r: 0.8745098039215686f32, g: 0.7764705882352941f32, b: 0.5215686274509804f32, a: 1f32};
// pub const 3AmInShibuya: Colour = Colour {r: 0.13333333333333333f32, g: 0.3333333333333333f32, b: 0.4666666666666667f32, a: 1f32};
// pub const 3AmLatte: Colour = Colour {r: 0.7529411764705882f32, g: 0.6627450980392157f32, b: 0.5568627450980392f32, a: 1f32};
// pub const 8BitEggplant: Colour = Colour {r: 0.6f32, g: 0.0f32, b: 0.4f32, a: 1f32};
pub const ADimeADozen: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.8666666666666667f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const ÀLOrange: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.5215686274509804f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const ASmellOfBakery: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9137254901960784f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const AStateOfMint: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 1.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const AbandonedSpaceship: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.47843137254901963f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const Abloom: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.796078431372549f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Abyssal: Colour = Colour {
    r: 0.25098039215686274f32,
    g: 0.2980392156862745f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const AbyssalWaters: Colour = Colour {
    r: 0.0f32,
    g: 0.3411764705882353f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const AcapulcoDive: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.6549019607843137f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Acid: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const AcidGreen: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.996078431372549f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const Acorn: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.3686274509803922f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const AcousticBrown: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.4196078431372549f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const ActiveVolcano: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const AdmiralBlue: Colour = Colour {
    r: 0.3137254901960784f32,
    g: 0.39215686274509803f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const AdriftOnTheNile: Colour = Colour {
    r: 0.5764705882352941f32,
    g: 0.7215686274509804f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const AfterMidnight: Colour = Colour {
    r: 0.2196078431372549f32,
    g: 0.2235294117647059f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const AfterTheStorm: Colour = Colour {
    r: 0.2f32,
    g: 0.3803921568627451f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const AgedAntics: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.4196078431372549f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const AgressiveAqua: Colour = Colour {
    r: 0.0f32,
    g: 0.984313725490196f32,
    b: 1.0f32,
    a: 1f32,
};
pub const AhoyBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.5098039215686274f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const Airkiss: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8627450980392157f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const Airborne: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.7607843137254902f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const AkariRed: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.043137254901960784f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const Alarm: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.0f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const Alaska: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8549019607843137f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const AlienAbduction: Colour = Colour {
    r: 0.047058823529411764f32,
    g: 1.0f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const Alienated: Colour = Colour {
    r: 0.0f32,
    g: 0.8f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const AlleyCat: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.40784313725490196f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const Alligator: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.4f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Almond: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8627450980392157f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const AlmondMilk: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.788235294117647f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const AloeTip: Colour = Colour {
    r: 0.5411764705882353f32,
    g: 0.5803921568627451f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const AloeVera: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.5294117647058824f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const AloneInTheDark: Colour = Colour {
    r: 0.0f32,
    g: 0.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Alpaca: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8980392156862745f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const AlpineExpedition: Colour = Colour {
    r: 0.6f32,
    g: 0.9333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Aluminium: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.5294117647058824f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const AlwaysGreenGrass: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Amaretto: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.43529411764705883f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const AmarettoSour: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.596078431372549f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const Amazon: Colour = Colour {
    r: 0.2196078431372549f32,
    g: 0.4823529411764706f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Amber: Colour = Colour {
    r: 1.0f32,
    g: 0.7490196078431373f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Ambrosia: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.8823529411764706f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const AmbrosialOceanside: Colour = Colour {
    r: 0.2784313725490196f32,
    g: 0.6823529411764706f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const AmnesiacWhite: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.984313725490196f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const Amor: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.2f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const AmoraPurple: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Amore: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.1843137254901961f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const Anarchist: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.18823529411764706f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Anchovy: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.43529411764705883f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const AncientPine: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.29411764705882354f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const AncientScroll: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8941176470588236f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const AndromedaBlue: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.803921568627451f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const AngelWing: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8745098039215686f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const AngelSTrumpet: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8666666666666667f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const AngelicWhite: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.9294117647058824f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const Anger: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.0f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const AngryFlamingo: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.3058823529411765f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const AngryGhost: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const AngryPasta: Colour = Colour {
    r: 1.0f32,
    g: 0.8f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const AngryTomato: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.12549019607843137f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const AnimalKingdom: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.7529411764705882f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const AnimeBlush: Colour = Colour {
    r: 1.0f32,
    g: 0.47843137254901963f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const AniseBiscotti: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.7294117647058823f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const AnnaBanana: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.8352941176470589f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const Annatto: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.3254901960784314f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const AnotherOneBitesTheDust: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.7333333333333333f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const AntarcticLove: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8705882352941177f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Anthracite: Colour = Colour {
    r: 0.1568627450980392f32,
    g: 0.1568627450980392f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const Antique: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.5176470588235295f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const AntiqueBrass: Colour = Colour {
    r: 0.4235294117647059f32,
    g: 0.27450980392156865f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const AntiquePortWine: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.12941176470588237f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const Aphrodisiac: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.35294117647058826f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const AphroditeAqua: Colour = Colour {
    r: 0.27058823529411763f32,
    g: 0.9137254901960784f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const AphroditeanFuchsia: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.0784313725490196f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const ApocalypticOrange: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.44313725490196076f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const ApolloLanding: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8980392156862745f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const AppleCherry: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.0784313725490196f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const Apricot: Colour = Colour {
    r: 1.0f32,
    g: 0.6941176470588235f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const ApricotFreeze: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8117647058823529f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const ApricotHaze: Colour = Colour {
    r: 1.0f32,
    g: 0.6666666666666666f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const ApricotSherbet: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.803921568627451f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const Apricotta: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.6431372549019608f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const AprilShowers: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8705882352941177f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const Aqua: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const AquaFiesta: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.8862745098039215f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const Aquamarine: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.9098039215686274f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Aquarius: Colour = Colour {
    r: 0.17647058823529413f32,
    g: 0.6901960784313725f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const AquaticEdge: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.8392156862745098f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const ArabicaMint: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 1.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Arancio: Colour = Colour {
    r: 1.0f32,
    g: 0.4392156862745098f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const ArcadeFire: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.2f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const ArcaneRed: Colour = Colour {
    r: 0.41568627450980394f32,
    g: 0.1843137254901961f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const Archeology: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.41568627450980394f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const Arctic: Colour = Colour {
    r: 0.39215686274509803f32,
    g: 0.5215686274509804f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const ArcticIce: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.7411764705882353f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const ArcticWater: Colour = Colour {
    r: 0.0f32,
    g: 0.9882352941176471f32,
    b: 0.9882352941176471f32,
    a: 1f32,
};
pub const ArganOil: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.4f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const Argent: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Argento: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.792156862745098f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const Ariel: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.8431372549019608f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const AristocraticVelvet: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.043137254901960784f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Armadillo: Colour = Colour {
    r: 0.2823529411764706f32,
    g: 0.2901960784313726f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const Armor: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.5215686274509804f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const AromaticHerbs: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.788235294117647f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const AroundTheGills: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.7137254901960784f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const Arrowwood: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.5254901960784314f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const Artichoke: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.592156862745098f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const ArtisansGold: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.6705882352941176f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const ArtistSCharcoal: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.2235294117647059f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const Arugula: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.6784313725490196f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const Ash: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.7294117647058823f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const AshesToAshes: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.7019607843137254f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const Asparagus: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.6705882352941176f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const Asphalt: Colour = Colour {
    r: 0.07450980392156863f32,
    g: 0.0392156862745098f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const Assassin: Colour = Colour {
    r: 0.17647058823529413f32,
    g: 0.30980392156862746f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const AssassinSRed: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.00784313725490196f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const Aster: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.4823529411764706f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const Astral: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.43529411764705883f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const AtlanticNavy: Colour = Colour {
    r: 0.07450980392156863f32,
    g: 0.2f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const Atlantis: Colour = Colour {
    r: 0.2f32,
    g: 0.3803921568627451f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const AtlasCedar: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.6274509803921569f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const AtomicLime: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 1.0f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const AtomicOrange: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.5254901960784314f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const AtomicPink: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.49411764705882355f32,
    b: 0.9921568627450981f32,
    a: 1f32,
};
pub const AtomicTangerine: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Aubergine: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.1450980392156863f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const AugustusAsparagus: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.6666666666666666f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const Aurora: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8196078431372549f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const AutumnCrocodile: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.4666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const AutumnFire: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.3058823529411765f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const AutumnGold: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.3843137254901961f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const Autumnal: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.34901960784313724f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const AvantgardePink: Colour = Colour {
    r: 1.0f32,
    g: 0.4666666666666667f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Avocado: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.5098039215686274f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const AvocadoStone: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.24313725490196078f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const AwardWinningWhite: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9411764705882353f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const AwkwardPurple: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.03137254901960784f32,
    b: 0.8f32,
    a: 1f32,
};
pub const AyahuascaVine: Colour = Colour {
    r: 0.4f32,
    g: 0.3333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Aztec: Colour = Colour {
    r: 0.1607843137254902f32,
    g: 0.20392156862745098f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const AztecTemple: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.4392156862745098f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const AztecWarrior: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Azulado: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.11372549019607843f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const Azure: Colour = Colour {
    r: 0.0f32,
    g: 0.4980392156862745f32,
    b: 1.0f32,
    a: 1f32,
};
pub const BabaGanoush: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Babe: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.4823529411764706f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const BabyBear: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.34901960784313724f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const BabyBlue: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.8117647058823529f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const BabyPink: Colour = Colour {
    r: 1.0f32,
    g: 0.7176470588235294f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const BackInBlack: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.0784313725490196f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const Backyard: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.596078431372549f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const BaconStrips: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.24705882352941178f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const BadassGrass: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.8549019607843137f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const BakerSBread: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.7019607843137254f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const BakerSDream: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.5607843137254902f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const BakeryBrown: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.5647058823529412f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const Baklava: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.7058823529411765f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const Ballerina: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8117647058823529f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const Ballet: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8352941176470589f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const Balsamico: Colour = Colour {
    r: 0.07450980392156863f32,
    g: 0.050980392156862744f32,
    b: 0.027450980392156862f32,
    a: 1f32,
};
pub const Baltic: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.615686274509804f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const BalticAmber: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.7176470588235294f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Bambino: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.8549019607843137f32,
    b: 0.8f32,
    a: 1f32,
};
pub const BambooForest: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.6627450980392157f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const Bananappeal: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9529411764705882f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const Banana: Colour = Colour {
    r: 1.0f32,
    g: 0.9882352941176471f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const BananaBandanna: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9686274509803922f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const BananaBombshell: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9098039215686274f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const BananaBread: Colour = Colour {
    r: 1.0f32,
    g: 0.8117647058823529f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const BananaBrick: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8470588235294118f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const BananaCream: Colour = Colour {
    r: 1.0f32,
    g: 0.9568627450980393f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const BananaFlash: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.996078431372549f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const BananaFrappé: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8352941176470589f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const BananaKing: Colour = Colour {
    r: 1.0f32,
    g: 0.984313725490196f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const BananaMania: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9058823529411765f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const BananaMilk: Colour = Colour {
    r: 1.0f32,
    g: 0.9686274509803922f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const BananaPepper: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8392156862745098f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const BananaPropaganda: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8588235294117647f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BananaRepublic: Colour = Colour {
    r: 1.0f32,
    g: 0.8862745098039215f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const BananaSplit: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9333333333333333f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const Bancha: Colour = Colour {
    r: 0.4f32,
    g: 0.41568627450980394f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const BaneOfRoyalty: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.0784313725490196f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Bangalore: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const BankVault: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.45098039215686275f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const BannerGold: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.5215686274509804f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const Barbara: Colour = Colour {
    r: 1.0f32,
    g: 0.058823529411764705f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const Barbarossa: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.2784313725490196f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Barbecue: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.3803921568627451f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const Barbera: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.011764705882352941f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const Barberry: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Bark: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.34509803921568627f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Barolo: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.0f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const BarrelAged: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.4117647058823529f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const Basil: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.6235294117647059f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const BasilSmash: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.8823529411764706f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const Basketball: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.403921568627451f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const BasswoodGreen: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.6196078431372549f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const BatWing: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.4549019607843137f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Batsignal: Colour = Colour {
    r: 0.996078431372549f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BatSBloodSoup: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.2f32,
    b: 0.4f32,
    a: 1f32,
};
pub const BathBubbles: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.9490196078431372f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const BathWater: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Batman: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.43137254901960786f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const BatsCloak: Colour = Colour {
    r: 0.12156862745098039f32,
    g: 0.08235294117647059f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const Battletoad: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.8f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const BavarianGreen: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.6039215686274509f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Bay: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.8862745098039215f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const BayView: Colour = Colour {
    r: 0.41568627450980394f32,
    g: 0.5058823529411764f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const BeachDune: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.7333333333333333f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const BeachGlass: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.8745098039215686f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const BeachView: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.4627450980392157f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const BeachesOfCancun: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9294117647058824f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const BearHug: Colour = Colour {
    r: 0.4745098039215686f32,
    g: 0.38823529411764707f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const BeatAroundTheBush: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.41568627450980394f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const BeauMonde: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.7019607843137254f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const BeauVert: Colour = Colour {
    r: 0.047058823529411764f32,
    g: 0.3764705882352941f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const BeautifulDarkness: Colour = Colour {
    r: 0.40784313725490196f32,
    g: 0.42745098039215684f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const BeautyAndTheBeach: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.5882352941176471f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const Béchamel: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.9333333333333333f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const Becquerel: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.9254901960784314f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const BeeYellow: Colour = Colour {
    r: 0.996078431372549f32,
    g: 1.0f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const Beekeeper: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8941176470588236f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const Beer: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.6666666666666666f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const Beeswax: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.8431372549019608f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const BeetRed: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.12549019607843137f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const BeetrootPurple: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.2f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const Begonia: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.43137254901960786f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const Beige: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8549019607843137f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const BeigeAndSage: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.7568627450980392f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Bejewelled: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.6352941176470588f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const BelgianWaffle: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8745098039215686f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const BellTower: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8156862745098039f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Belladonna: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Bellflower: Colour = Colour {
    r: 0.36470588235294116f32,
    g: 0.4f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Bellini: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.788235294117647f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const BelovedSunflower: Colour = Colour {
    r: 1.0f32,
    g: 0.7294117647058823f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const BelowTheSurface: Colour = Colour {
    r: 0.0392156862745098f32,
    g: 0.1843137254901961f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const BelowZero: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.803921568627451f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Beluga: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9490196078431372f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const BenevolentPink: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Bengal: Colour = Colour {
    r: 0.8f32,
    g: 0.592156862745098f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const BentoBox: Colour = Colour {
    r: 0.8f32,
    g: 0.21176470588235294f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const Berber: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.8117647058823529f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const Bergamot: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.7803921568627451f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const Bermuda: Colour = Colour {
    r: 0.10588235294117647f32,
    g: 0.49019607843137253f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const BermudaGrass: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.6235294117647059f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const BermudaOnion: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.35294117647058826f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const BerriesNCream: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.7215686274509804f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const Berry: Colour = Colour {
    r: 0.6f32,
    g: 0.058823529411764705f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const BerryBlast: Colour = Colour {
    r: 1.0f32,
    g: 0.00392156862745098f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const BerryButter: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.807843137254902f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const BerryGood: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.7647058823529411f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const BerryJam: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.34509803921568627f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const Berrylicious: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.3686274509803922f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const BethlehemSuperstar: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9333333333333333f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const BeurreBlanc: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8823529411764706f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const BeveledGlass: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.8f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const BeyondThePines: Colour = Colour {
    r: 0.40784313725490196f32,
    g: 0.5019607843137255f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const BeyondTheStars: Colour = Colour {
    r: 0.0392156862745098f32,
    g: 0.19607843137254902f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Bianca: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.9372549019607843f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const BigBangPink: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.6f32,
    a: 1f32,
};
pub const BigFishToFry: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8588235294117647f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const BigSpender: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.8666666666666667f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const BigYellowTaxi: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Bigfoot: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.3176470588235294f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const Billiard: Colour = Colour {
    r: 0.0f32,
    g: 0.6862745098039216f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const BindiRed: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.0f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const BiohazardSuit: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.984313725490196f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const Bioshock: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.6f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BirchWhite: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9333333333333333f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const BirdBathBlue: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.8745098039215686f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Biscuit: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9294117647058824f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const BiscuitDough: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8588235294117647f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const Bison: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.30980392156862746f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const Bisque: Colour = Colour {
    r: 1.0f32,
    g: 0.8941176470588236f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const BiteMyTongue: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.49019607843137253f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const BitterChocolate: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.1607843137254902f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const BitterLemon: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.8588235294117647f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const BitterLiquorice: Colour = Colour {
    r: 0.14901960784313725f32,
    g: 0.1607843137254902f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const BitterMelon: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.8196078431372549f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const Bittersweet: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.6274509803921569f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Black: Colour = Colour {
    r: 0.0f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BlackBox: Colour = Colour {
    r: 0.058823529411764705f32,
    g: 0.1568627450980392f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const BlackCherry: Colour = Colour {
    r: 0.17254901960784313f32,
    g: 0.08627450980392157f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const BlackChocolate: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BlackForest: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.38823529411764707f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const BlackHole: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.00784313725490196f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const BlackKnight: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.043137254901960784f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const BlackMagic: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.27058823529411763f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const BlackMana: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.5215686274509804f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const BlackMetal: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.023529411764705882f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const BlackOlive: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.23529411764705882f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const BlackOrchid: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.32941176470588235f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const BlackOut: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const BlackPanther: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.25882352941176473f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const BlackPearl: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.15294117647058825f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const BlackPower: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.29411764705882354f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const BlackSabbath: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const BlackSheep: Colour = Colour {
    r: 0.058823529411764705f32,
    g: 0.050980392156862744f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const BlackStallion: Colour = Colour {
    r: 0.054901960784313725f32,
    g: 0.09803921568627451f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const BlackTruffle: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.23921568627450981f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const BlackTurmeric: Colour = Colour {
    r: 0.17254901960784313f32,
    g: 0.2627450980392157f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const BlackVelvet: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const BlackWash: Colour = Colour {
    r: 0.047058823529411764f32,
    g: 0.047058823529411764f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const Blackberry: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.09411764705882353f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const BlackberryYogurt: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.7411764705882353f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const BlacknT: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.058823529411764705f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const Blackout: Colour = Colour {
    r: 0.054901960784313725f32,
    g: 0.027450980392156862f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const Blackwater: Colour = Colour {
    r: 0.32941176470588235f32,
    g: 0.33725490196078434f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const BlancCassé: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9333333333333333f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const Blanco: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.9176470588235294f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const BlankCanvas: Colour = Colour {
    r: 1.0f32,
    g: 0.9372549019607843f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const BlankStare: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.611764705882353f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const BlasphemousBlue: Colour = Colour {
    r: 0.2f32,
    g: 0.33725490196078434f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const BlazingDragonfruit: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const BleachedOlive: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const BleachedSunflower: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9098039215686274f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const BleedingCrimson: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.0784313725490196f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const BleedingHeart: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.1803921568627451f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const BleuCiel: Colour = Colour {
    r: 0.0f32,
    g: 0.4823529411764706f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const BlinkingTerminal: Colour = Colour {
    r: 0.4f32,
    g: 0.8f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BlissfulSerenity: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9333333333333333f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const BlisterPearl: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 1.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const BlizzardBlue: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.8901960784313725f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Blond: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9411764705882353f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const BloodBrother: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const BloodBurst: Colour = Colour {
    r: 1.0f32,
    g: 0.2784313725490196f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const BloodDonor: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.09411764705882353f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const BloodKiss: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.043137254901960784f32,
    b: 0.0392156862745098f32,
    a: 1f32,
};
pub const BloodOrange: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.0f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const BloodRush: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.13333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Bloodhound: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Bloodthirsty: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const BloodthirstyBeige: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8431372549019608f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const BloodthirstyLips: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.06274509803921569f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const BloodyMary: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.00392156862745098f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const BloodySalmon: Colour = Colour {
    r: 0.8f32,
    g: 0.26666666666666666f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Blossom: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9137254901960784f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const BlossomingDynasty: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.3254901960784314f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const BlowingKisses: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8705882352941177f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const Blue: Colour = Colour {
    r: 0.0f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const BlueBay: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.6039215686274509f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const BlueBell: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.6862745098039216f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const BlueBikini: Colour = Colour {
    r: 0.0f32,
    g: 0.7333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const BlueBlood: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.4980392156862745f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const BlueBobbin: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.7058823529411765f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const BlueChip: Colour = Colour {
    r: 0.11372549019607843f32,
    g: 0.33725490196078434f32,
    b: 0.6f32,
    a: 1f32,
};
pub const BlueHaze: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.7294117647058823f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const BlueHour: Colour = Colour {
    r: 0.0f32,
    g: 0.20392156862745098f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const BlueLips: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.7372549019607844f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const BlueMana: Colour = Colour {
    r: 0.40784313725490196f32,
    g: 0.7607843137254902f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const BlueMartini: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.7058823529411765f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const BlueMoon: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.5725490196078431f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const BlueOverdose: Colour = Colour {
    r: 0.0f32,
    g: 0.12549019607843137f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const BlueRibbon: Colour = Colour {
    r: 0.0f32,
    g: 0.4f32,
    b: 1.0f32,
    a: 1f32,
};
pub const BlueScreenOfDeath: Colour = Colour {
    r: 0.0f32,
    g: 0.2f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const BlueSilk: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.8627450980392157f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const BlueTriumph: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.4627450980392157f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const BlueVelvet: Colour = Colour {
    r: 0.050980392156862744f32,
    g: 0.3803921568627451f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const BlueWhale: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.20392156862745098f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const Bluebell: Colour = Colour {
    r: 0.2f32,
    g: 0.2f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Blueberry: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.2549019607843137f32,
    b: 0.5882352941176471f32,
    a: 1f32,
};
pub const Bluebonnet: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.10980392156862745f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const Bluerocratic: Colour = Colour {
    r: 0.12156862745098039f32,
    g: 0.4f32,
    b: 1.0f32,
    a: 1f32,
};
pub const BluesWhiteShoes: Colour = Colour {
    r: 0.6f32,
    g: 0.7294117647058823f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Bluetiful: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.4117647058823529f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const BlushBomb: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const BlushDAmour: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.36470588235294116f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const BlushKiss: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.7372549019607844f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const BlushRush: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.7372549019607844f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const BlushingCinnamon: Colour = Colour {
    r: 1.0f32,
    g: 0.7490196078431373f32,
    b: 0.6f32,
    a: 1f32,
};
pub const BlushingCoconut: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8352941176470589f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const BlushingRose: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.6078431372549019f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const BlusteringBlue: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.06666666666666667f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Boa: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.4666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const BohemianBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.0f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const BohoBlush: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.5294117647058824f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const BohoCopper: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.3764705882352941f32,
    b: 0.2f32,
    a: 1f32,
};
pub const BoilingMagma: Colour = Colour {
    r: 1.0f32,
    g: 0.2f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BokChoy: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.792156862745098f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const BoldIrish: Colour = Colour {
    r: 0.16470588235294117f32,
    g: 0.5058823529411764f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const BollywoodGold: Colour = Colour {
    r: 1.0f32,
    g: 0.984313725490196f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const Bolognese: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.26666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Bone: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.8431372549019608f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const Bonechilling: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.9490196078431372f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const Bonfire: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.5019607843137255f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const BonneNuit: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.2823529411764706f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Bonsai: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.4823529411764706f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const BonsaiGarden: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.6196078431372549f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const Bookworm: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8901960784313725f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const Bordeaux: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.0f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const BorderlinePink: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const BorlottiBean: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.6941176470588235f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Botanical: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.43137254901960786f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const BotanicalGarden: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.6666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const BotanicalNight: Colour = Colour {
    r: 0.07058823529411765f32,
    g: 0.25098039215686274f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const BottomOfMyHeart: Colour = Colour {
    r: 0.8f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Boulevardier: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.027450980392156862f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const Bourbon: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.4235294117647059f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const BourbonPeach: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.5176470588235295f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const BoutiqueBeige: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.807843137254902f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const BrainFreeze: Colour = Colour {
    r: 0.0f32,
    g: 0.9333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Brandy: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.7137254901960784f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const BrandyBear: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.32941176470588235f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const BrandywineSpritz: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.615686274509804f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const Brass: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.6509803921568628f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const BrassButtons: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.6745098039215687f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const BreadAndButter: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9294117647058824f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const BreadCrumb: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.8313725490196079f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const BreakTheIce: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.8823529411764706f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const BreathOfCelery: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.9058823529411765f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const BreathOfFire: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const BreathOfFreshAir: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.8588235294117647f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const Breeze: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.8666666666666667f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const BreezeOfChilli: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.4392156862745098f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const Brick: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.21176470588235294f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const BrickRed: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.0784313725490196f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const BrickyBrick: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.22745098039215686f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Bride: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9058823529411765f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const BrideSBlush: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.8862745098039215f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const BrightStar: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8862745098039215f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Brilliance: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9921568627450981f32,
    b: 0.9921568627450981f32,
    a: 1f32,
};
pub const BrilliantBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.4588235294117647f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const BrilliantGold: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8588235294117647f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const BrinkPink: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.3764705882352941f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const Brisket: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.27058823529411763f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const BritishPhoneBooth: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const BroadDaylight: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8666666666666667f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Broccoli: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.7019607843137254f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const BroccoliGreen: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.3254901960784314f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const BroccoliParadise: Colour = Colour {
    r: 0.0f32,
    g: 0.5333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Bronze: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.4745098039215686f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Bronzed: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.4f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Broom: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const Brown: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.21568627450980393f32,
    b: 0.0f32,
    a: 1f32,
};
pub const BrownAlpaca: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.42745098039215684f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const BrownCoffee: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.17254901960784313f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const BrownSugar: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.4627450980392157f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const BrownSugarGlaze: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.47843137254901963f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Brownie: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.29411764705882354f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Bruise: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.25098039215686274f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const BruisedPlum: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.09803921568627451f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const Brume: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.7764705882352941f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const Brunette: Colour = Colour {
    r: 0.4f32,
    g: 0.25882352941176473f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const Bruschetta: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.396078431372549f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const BrutalPink: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const BubbelgumHeart: Colour = Colour {
    r: 1.0f32,
    g: 0.7294117647058823f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const BubbleGum: Colour = Colour {
    r: 1.0f32,
    g: 0.5215686274509804f32,
    b: 1.0f32,
    a: 1f32,
};
pub const BubblegumBabyGirl: Colour = Colour {
    r: 0.8f32,
    g: 0.3333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const BubblegumCrisis: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const BubblegumKisses: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.5725490196078431f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const Bubbles: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.996078431372549f32,
    b: 1.0f32,
    a: 1f32,
};
pub const BüchelCherry: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Buckeye: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.2823529411764706f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Bucolic: Colour = Colour {
    r: 0.10588235294117647f32,
    g: 0.4f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const BudGreen: Colour = Colour {
    r: 0.4745098039215686f32,
    g: 0.7058823529411765f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const BuddhaSLoveHandles: Colour = Colour {
    r: 1.0f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const BuffIt: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.8117647058823529f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const BulletHell: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9450980392156862f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const Bullfrog: Colour = Colour {
    r: 0.5411764705882353f32,
    g: 0.5882352941176471f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const BulmaHair: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.6196078431372549f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const Bumblebee: Colour = Colour {
    r: 1.0f32,
    g: 0.7843137254901961f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const BunnyTail: Colour = Colour {
    r: 1.0f32,
    g: 0.8901960784313725f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const Bureaucracy: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.4235294117647059f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const BuriedGold: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.7372549019607844f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const BurningFireflies: Colour = Colour {
    r: 1.0f32,
    g: 0.06666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const BurningFlame: Colour = Colour {
    r: 1.0f32,
    g: 0.6941176470588235f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const BurningOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.44313725490196076f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const BurningRaspberry: Colour = Colour {
    r: 1.0f32,
    g: 0.0196078431372549f32,
    b: 0.6f32,
    a: 1f32,
};
pub const BurningTrail: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const BurningUltrablue: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.0392156862745098f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const BurntCoffee: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.10588235294117647f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const BurntRed: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.13725490196078433f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const Burrito: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8431372549019608f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const BustyBlue: Colour = Colour {
    r: 0.2f32,
    g: 0.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const BusyBee: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Butter: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const ButterBronze: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.5333333333333333f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const ButterHoney: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.8980392156862745f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const ButterMuffin: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8745098039215686f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const ButterUp: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8784313725490196f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Butterbeer: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.4745098039215686f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Buttercream: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8784313725490196f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const ButteredPopcorn: Colour = Colour {
    r: 1.0f32,
    g: 0.9411764705882353f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const ButteredUp: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9411764705882353f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const ButterflyKisses: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8705882352941177f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const Buttermelon: Colour = Colour {
    r: 1.0f32,
    g: 0.9686274509803922f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const Buttermilk: Colour = Colour {
    r: 1.0f32,
    g: 0.996078431372549f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const Butternut: Colour = Colour {
    r: 1.0f32,
    g: 0.6313725490196078f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const ButternutSquash: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.4627450980392157f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const Butterscotch: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.6941176470588235f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const ButterscotchCake: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.7843137254901961f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Butterum: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.5607843137254902f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const Buttery: Colour = Colour {
    r: 1.0f32,
    g: 0.7607843137254902f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const ButteryCroissant: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8823529411764706f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const Buzz: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.7764705882352941f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const C64Blue: Colour = Colour {
    r: 0.0f32,
    g: 0.22745098039215686f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Cabaret: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.3215686274509804f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Cabbage: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.8431372549019608f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const CacaoNibs: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.26666666666666666f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const CacodemonRed: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Cactus: Colour = Colour {
    r: 0.3568627450980392f32,
    g: 0.43529411764705883f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const CadillacCoupe: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.21176470588235294f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const CaduceusStaff: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const CaféAuLait: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.48627450980392156f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const CaféCrème: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.5882352941176471f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const CafeLatte: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.7764705882352941f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const CaféNoir: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.21176470588235294f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const CafeRoyale: Colour = Colour {
    r: 0.41568627450980394f32,
    g: 0.28627450980392155f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const Cajeta: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.42745098039215684f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const CakeFrosting: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.8745098039215686f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const CakepopSorbet: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.7764705882352941f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const Calabrese: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.6509803921568628f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const CalciumRock: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9137254901960784f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const CallItANight: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.21176470588235294f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const CalmWaters: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.9803921568627451f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const Calypso: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.44313725490196076f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Camel: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.6235294117647059f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const CamelCardinal: Colour = Colour {
    r: 0.8f32,
    g: 0.6f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Camellia: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.40784313725490196f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const Cameo: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8705882352941177f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const CamoClay: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.4980392156862745f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const Campanula: Colour = Colour {
    r: 0.20392156862745098f32,
    g: 0.45098039215686275f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const Campfire: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.37254901960784315f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const CampingTrip: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.47058823529411764f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const CanadianMaple: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.6980392156862745f32,
    b: 0.4f32,
    a: 1f32,
};
pub const CanadianTuxedo: Colour = Colour {
    r: 0.3411764705882353f32,
    g: 0.6039215686274509f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const Canary: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 1.0f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const CandiedApple: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.3568627450980392f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const CandiedSnow: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 1.0f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const CandleGlow: Colour = Colour {
    r: 1.0f32,
    g: 0.9098039215686274f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const CandleInTheWind: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9215686274509803f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Candlelight: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8509803921568627f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const Candy: Colour = Colour {
    r: 1.0f32,
    g: 0.6078431372549019f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const CandyAppleRed: Colour = Colour {
    r: 1.0f32,
    g: 0.03137254901960784f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CandyBar: Colour = Colour {
    r: 1.0f32,
    g: 0.7176470588235294f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const CandyCorn: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9882352941176471f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const CandyDreams: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.6823529411764706f32,
    b: 0.9490196078431372f32,
    a: 1f32,
};
pub const CandyFloss: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.6549019607843137f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const CandyGrapeFizz: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.3333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const CandyGreen: Colour = Colour {
    r: 0.2f32,
    g: 0.8f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CandyPink: Colour = Colour {
    r: 1.0f32,
    g: 0.38823529411764707f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const CaneSugar: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.7254901960784313f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const CannoliCream: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9254901960784314f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const Cantaloupe: Colour = Colour {
    r: 1.0f32,
    g: 0.8313725490196079f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const CanyonSunset: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.5333333333333333f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const Capers: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.47843137254901963f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const CapitalBlue: Colour = Colour {
    r: 0.10196078431372549f32,
    g: 0.2549019607843137f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const Caponata: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.16470588235294117f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const Cappuccino: Colour = Colour {
    r: 0.4392156862745098f32,
    g: 0.2901960784313726f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const CappuccinoCosmico: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8666666666666667f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Capri: Colour = Colour {
    r: 0.0f32,
    g: 0.7490196078431373f32,
    b: 1.0f32,
    a: 1f32,
};
pub const CaptainKirk: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.5294117647058824f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const Caramel: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.43529411764705883f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const CaramelCoating: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.4666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const CaramelCrumb: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.5764705882352941f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const CaramelDream: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.3843137254901961f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const CaramelDrizzle: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.6784313725490196f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const CaramelFinish: Colour = Colour {
    r: 1.0f32,
    g: 0.8352941176470589f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const CaramelGold: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.5764705882352941f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const CaramelMacchiato: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.5529411764705883f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const CaramelMousse: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.792156862745098f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const Caramelize: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.5411764705882353f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const CaramelizedPecan: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.4823529411764706f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const Carbon: Colour = Colour {
    r: 0.2f32,
    g: 0.2f32,
    b: 0.2f32,
    a: 1f32,
};
pub const CarbonFiber: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.1803921568627451f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const Cardamom: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.6666666666666666f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Cardboard: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.6039215686274509f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Cardinal: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.11764705882352941f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const Caribbean: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.9411764705882353f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const CaribbeanBlue: Colour = Colour {
    r: 0.10196078431372549f32,
    g: 0.7568627450980392f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Caribou: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.4235294117647059f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const Carmine: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.0f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const Carnivore: Colour = Colour {
    r: 0.6f32,
    g: 0.06666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const CarolinaReaper: Colour = Colour {
    r: 1.0f32,
    g: 0.08235294117647059f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Carona: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.6470588235294118f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const Carpaccio: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.25882352941176473f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Carrot: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.43529411764705883f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const CarrotLava: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.35294117647058826f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const CartoonViolence: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.09019607843137255f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const CascadeTwilight: Colour = Colour {
    r: 0.13725490196078433f32,
    g: 0.2823529411764706f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const Cascara: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.26666666666666666f32,
    b: 0.2f32,
    a: 1f32,
};
pub const CashewNut: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Cashmere: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.7019607843137254f32,
    b: 0.6f32,
    a: 1f32,
};
pub const CashmereClay: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.6352941176470588f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const CasinoLights: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9490196078431372f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Casper: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.7098039215686275f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const Castaway: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.7294117647058823f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const CastawayBeach: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.7568627450980392f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const CastleInTheSky: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.9176470588235294f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Castro: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.13725490196078433f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const CatacombWalls: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.8431372549019608f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const Catfish: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.49019607843137253f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Cathedral: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.6666666666666666f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const CathodeGreen: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Catnip: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.6666666666666666f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const Cauliflower: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8980392156862745f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const Caveman: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.3607843137254902f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Caviar: Colour = Colour {
    r: 0.16862745098039217f32,
    g: 0.17254901960784313f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const CavoloNero: Colour = Colour {
    r: 0.4470588235294118f32,
    g: 0.5764705882352941f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const Cayenne: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.06666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CedarChest: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.35294117647058826f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const Celadon: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.8823529411764706f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const CeladonPorcelain: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.7450980392156863f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const Celery: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.7529411764705882f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const CeleryMousse: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.9921568627450981f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const CeleryScepter: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8745098039215686f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const Celestial: Colour = Colour {
    r: 0.0f32,
    g: 0.47058823529411764f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const CelestialCathedral: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.9176470588235294f32,
    b: 0.9647058823529412f32,
    a: 1f32,
};
pub const CelestialHorizon: Colour = Colour {
    r: 0.48627450980392156f32,
    g: 0.5803921568627451f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const CementFeet: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.45098039215686275f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const Ceramic: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 1.0f32,
    b: 0.9764705882352941f32,
    a: 1f32,
};
pub const CerealFlake: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8431372549019608f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const Cerulean: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const ChaiLatte: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.796078431372549f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const ChaiTea: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.4823529411764706f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const ChainMail: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.4666666666666667f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const Chalet: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.596078431372549f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const Champagne: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.8235294117647058f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const ChampagneGold: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8392156862745098f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Channel: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.7647058823529411f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const Chantilly: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.7215686274509804f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const ChaoticRoses: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Charcoal: Colour = Colour {
    r: 0.20392156862745098f32,
    g: 0.2196078431372549f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const CharismaticRed: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Charm: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.4549019607843137f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const CharmingPeach: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.6784313725490196f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const Chartreuse: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.9725490196078431f32,
    b: 0.0392156862745098f32,
    a: 1f32,
};
pub const CheGuevaraRed: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.12941176470588237f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const Cheddar: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6039215686274509f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const CheekRed: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.35294117647058826f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const CheekyChestnut: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.30196078431372547f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const CheerlyKiwi: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.796078431372549f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const Cheese: Colour = Colour {
    r: 1.0f32,
    g: 0.6509803921568628f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CheeseItUp: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8705882352941177f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const CheesePlease: Colour = Colour {
    r: 1.0f32,
    g: 0.5882352941176471f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const Cheesecake: Colour = Colour {
    r: 1.0f32,
    g: 0.9882352941176471f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const Cheesus: Colour = Colour {
    r: 1.0f32,
    g: 0.8f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const CheesyCheetah: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6901960784313725f32,
    b: 0.2f32,
    a: 1f32,
};
pub const CheesyFrittata: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8784313725490196f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const ChefSHat: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9568627450980393f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const ChefSKiss: Colour = Colour {
    r: 0.8f32,
    g: 0.23137254901960785f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const Cherry: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.00784313725490196f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const CherryBerry: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.30196078431372547f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const CherryBlossom: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.7568627450980392f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const CherryBomb: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.23921568627450981f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const CherryCrush: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.0784313725490196f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const CherryPaddlePop: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.19215686274509805f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const CherryPicking: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.043137254901960784f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const CherrySangria: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.1411764705882353f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const CherrySoda: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const CherryTomato: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.00392156862745098f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const CherryVelvet: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.023529411764705882f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const Cherryade: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.15294117647058825f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const ChessIvory: Colour = Colour {
    r: 1.0f32,
    g: 0.9137254901960784f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const Chestnut: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.1568627450980392f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const ChewingGum: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.6901960784313725f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const Chickadee: Colour = Colour {
    r: 1.0f32,
    g: 0.8117647058823529f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const ChickenComb: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const ChickenMasala: Colour = Colour {
    r: 0.8f32,
    g: 0.5333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const ChickeryChick: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9137254901960784f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const ChildOfTheNight: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const ChiliConCarne: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.3686274509803922f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const ChiliCrab: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.22745098039215686f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const ChiliPepper: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.11764705882352941f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const ChillOfTheNight: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.42745098039215684f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const Chimera: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.3843137254901961f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const ChinaSilk: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.8196078431372549f32,
    b: 0.8f32,
    a: 1f32,
};
pub const ChineseNewYear: Colour = Colour {
    r: 1.0f32,
    g: 0.2f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Chinotto: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.2784313725490196f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const Chipmunk: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.6313725490196078f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const ChivalrousFox: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.4f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const ChivalrousWalrus: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.396078431372549f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const ChocoChic: Colour = Colour {
    r: 0.6f32,
    g: 0.2f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Chocoholic: Colour = Colour {
    r: 0.6f32,
    g: 0.2f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Chocolate: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.4117647058823529f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const ChocolateBells: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.3176470588235294f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const ChocolateBliss: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.3764705882352941f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const ChocolateCastle: Colour = Colour {
    r: 0.27058823529411763f32,
    g: 0.13333333333333333f32,
    b: 0.027450980392156862f32,
    a: 1f32,
};
pub const ChocolateChili: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.25882352941176473f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const ChocolateCovered: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.2549019607843137f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const ChocolateEscape: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.23921568627450981f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const ChocolateExplosion: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.2784313725490196f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const ChocolateFantasies: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.21176470588235294f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const ChocolateKiss: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.0784313725490196f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const ChocolateLust: Colour = Colour {
    r: 0.6f32,
    g: 0.2f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const ChocolateMagma: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.27450980392156865f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const ChocolatePretzel: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.3137254901960784f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const ChocolateRain: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.30980392156862746f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const ChocolateRush: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.10588235294117647f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const ChocolateTemptation: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.43137254901960786f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const ChocolateTruffle: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.1803921568627451f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const ChocolateVelvet: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.4549019607843137f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const Chorizo: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const ChouxÀLaCrème: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8117647058823529f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const ChristmasRed: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.10588235294117647f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const ChromeWhite: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7803921568627451f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const ChubbyKiss: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.20784313725490197f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const Chutney: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.3686274509803922f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Cigar: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.3058823529411765f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const CigaretteGlow: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Cinder: Colour = Colour {
    r: 0.1411764705882353f32,
    g: 0.16470588235294117f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const Cinderella: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.8431372549019608f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Cinnamon: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.4117647058823529f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const CinnamonBuff: Colour = Colour {
    r: 1.0f32,
    g: 0.7490196078431373f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const Cinnapink: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.39215686274509803f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const Citadel: Colour = Colour {
    r: 0.41568627450980394f32,
    g: 0.4980392156862745f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const Citrus: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.7176470588235294f32,
    b: 0.0392156862745098f32,
    a: 1f32,
};
pub const CitrusSplash: Colour = Colour {
    r: 1.0f32,
    g: 0.7686274509803922f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CityDweller: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.7254901960784313f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const Clairvoyant: Colour = Colour {
    r: 0.2823529411764706f32,
    g: 0.023529411764705882f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const ClamUp: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8588235294117647f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const ClassyMauve: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.6f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Clay: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.41568627450980394f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const ClearSky: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.8f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const ClearWater: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.8352941176470589f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const Clementine: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.43137254901960786f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CloakAndDagger: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.0f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const CloakGrey: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.3686274509803922f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Cloisonné: Colour = Colour {
    r: 0.027450980392156862f32,
    g: 0.45098039215686275f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const ClottedCream: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9372549019607843f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const CloudBreak: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9450980392156862f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const CloudDancer: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9333333333333333f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const Cloudless: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.9176470588235294f32,
    b: 0.9882352941176471f32,
    a: 1f32,
};
pub const CloudyValley: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.7764705882352941f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const Clover: Colour = Colour {
    r: 0.0f32,
    g: 0.5607843137254902f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Coalmine: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Cobalt: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.0392156862745098f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const CocaMocha: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.615686274509804f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const Cockatoo: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.7843137254901961f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const CocoMalt: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.8627450980392157f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const CocoMuck: Colour = Colour {
    r: 0.6f32,
    g: 0.2901960784313726f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const Cocoa: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.37254901960784315f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const Cocoloco: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.5607843137254902f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Coconut: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.35294117647058826f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const CoconutAgony: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.9098039215686274f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const CoconutMacaroon: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.792156862745098f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const CoconutMilk: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9215686274509803f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const Cocoon: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8588235294117647f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Coffee: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.3058823529411765f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const CoffeeAdept: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.3333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const CoffeeDiva: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.6588235294117647f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const Cognac: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.5490196078431373f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const CoinSlot: Colour = Colour {
    r: 1.0f32,
    g: 0.26666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Cola: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.1843137254901961f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const ColdAndDark: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.25882352941176473f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const ColdBlue: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const ColdCanada: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 1.0f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const ColdLightOfDay: Colour = Colour {
    r: 0.0f32,
    g: 0.9333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const ColdLips: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.6274509803921569f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const ColdPressCoffee: Colour = Colour {
    r: 0.4235294117647059f32,
    g: 0.1803921568627451f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const ColdShoulder: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.8784313725490196f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const ColdTurkey: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7098039215686275f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const ColdWave: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.8862745098039215f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const ColdWhite: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9882352941176471f32,
    b: 0.984313725490196f32,
    a: 1f32,
};
pub const ColumboSCoat: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.796078431372549f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const Communist: Colour = Colour {
    r: 0.8f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Concord: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.4980392156862745f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const Concrete: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.8196078431372549f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const ConcreteJungle: Colour = Colour {
    r: 0.6f32,
    g: 0.6f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const ConcreteLandscape: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.3764705882352941f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const Conifer: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.8666666666666667f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const Conker: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.3058823529411765f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const ContinentalWaters: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.7764705882352941f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const CookieCrumb: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.592156862745098f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const CookieCrust: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.6980392156862745f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const CookieDough: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.44313725490196076f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Cool: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.7019607843137254f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const CoolAsACucumber: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.8470588235294118f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const CoolerThanEver: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.7333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Copacabana: Colour = Colour {
    r: 0.0f32,
    g: 0.4235294117647059f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const CopiousCaramel: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.5215686274509804f32,
    b: 0.11372549019607843f32,
    a: 1f32,
};
pub const Copper: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.45098039215686275f32,
    b: 0.2f32,
    a: 1f32,
};
pub const CopperCoin: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.5411764705882353f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const CopperHopper: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.25098039215686274f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CopperPatina: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.7058823529411765f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const Copperhead: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.5294117647058824f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Coquelicot: Colour = Colour {
    r: 1.0f32,
    g: 0.2196078431372549f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Coral: Colour = Colour {
    r: 1.0f32,
    g: 0.4980392156862745f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const CoralCommander: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4f32,
    b: 0.4f32,
    a: 1f32,
};
pub const CoralKiss: Colour = Colour {
    r: 1.0f32,
    g: 0.8666666666666667f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const CoralParadise: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.4f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const CoralRed: Colour = Colour {
    r: 1.0f32,
    g: 0.25098039215686274f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const Coralistic: Colour = Colour {
    r: 1.0f32,
    g: 0.5686274509803921f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Corbeau: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const CorfuWaters: Colour = Colour {
    r: 0.0f32,
    g: 0.5411764705882353f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const CorkWood: Colour = Colour {
    r: 0.8f32,
    g: 0.4666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Corn: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9254901960784314f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const Cornflake: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9019607843137255f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const Cornsilk: Colour = Colour {
    r: 1.0f32,
    g: 0.9725490196078431f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const Corona: Colour = Colour {
    r: 1.0f32,
    g: 0.7058823529411765f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const CorrosiveGreen: Colour = Colour {
    r: 0.32941176470588235f32,
    g: 0.8509803921568627f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const Cortex: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.5843137254901961f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const Cosmic: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.7254901960784313f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const CosmicBitFlip: Colour = Colour {
    r: 0.0f32,
    g: 0.06274509803921569f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CosmicExplorer: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const CosmicGreen: Colour = Colour {
    r: 0.18823529411764706f32,
    g: 0.6588235294117647f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const CosmicHeart: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.00392156862745098f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const CosmicLatte: Colour = Colour {
    r: 1.0f32,
    g: 0.9725490196078431f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const CosmicRed: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.1411764705882353f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const CotingaPurple: Colour = Colour {
    r: 0.20392156862745098f32,
    g: 0.0f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const CottonBall: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9686274509803922f32,
    b: 0.9921568627450981f32,
    a: 1f32,
};
pub const CottonBoll: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.9372549019607843f32,
    b: 0.984313725490196f32,
    a: 1f32,
};
pub const CottonCandy: Colour = Colour {
    r: 1.0f32,
    g: 0.7372549019607844f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const CottonCandyExplosions: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.13333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const CottonField: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9411764705882353f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const Countryside: Colour = Colour {
    r: 0.6431372549019608f32,
    g: 0.6431372549019608f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const Courtbouillon: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.796078431372549f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const Couscous: Colour = Colour {
    r: 1.0f32,
    g: 0.8862745098039215f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Cousteau: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.6627450980392157f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const CoverOfNight: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.3058823529411765f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const CowSMilk: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9294117647058824f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const Cowboy: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.21568627450980393f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const Coyote: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.6078431372549019f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const CozySummerSunset: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.6235294117647059f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const CozyWool: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.7254901960784313f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Cranberry: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.0f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const CranberrySplash: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.3215686274509804f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const CrashDummy: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const CrashingWaves: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.43529411764705883f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const Cream: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const CreamAndButter: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9333333333333333f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const CreamAndSugar: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8117647058823529f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const CreamPuff: Colour = Colour {
    r: 1.0f32,
    g: 0.7333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const CreamedCaramel: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.611764705882353f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const Creamy: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9098039215686274f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const CreamyApricot: Colour = Colour {
    r: 1.0f32,
    g: 0.9098039215686274f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const CreamyAvocado: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.9450980392156862f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const CreamyBerry: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.7372549019607844f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const CreamyCloudDreams: Colour = Colour {
    r: 1.0f32,
    g: 0.9607843137254902f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const CreamyGarlic: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.9372549019607843f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const CreamyIvory: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const CreamyLemon: Colour = Colour {
    r: 1.0f32,
    g: 0.9411764705882353f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const CreamyMint: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 1.0f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const CreamyPeach: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.6392156862745098f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const CreamyStrawberry: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8235294117647058f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const CreamySweetCorn: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.7647058823529411f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const CreamyVanilla: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8980392156862745f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const CrèmeBrûlée: Colour = Colour {
    r: 1.0f32,
    g: 0.8901960784313725f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const CrèmeDeLaCrème: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9058823529411765f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const CrèmeDePêche: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9607843137254902f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const CrèmeFraîche: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.9333333333333333f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Creole: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.19607843137254902f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const Crepe: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.7372549019607844f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const CrimsonBlaze: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.23921568627450981f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const CrimsonBoy: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.28627450980392155f32,
    b: 0.2f32,
    a: 1f32,
};
pub const CrimsonCloud: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.1843137254901961f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const CrimsonGlow: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.2235294117647059f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const CrimsonVelvetSunset: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.14901960784313725f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const Crisps: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.7411764705882353f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const CrispyCrunch: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.5607843137254902f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const Croissant: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.6705882352941176f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const Crow: Colour = Colour {
    r: 0.09411764705882353f32,
    g: 0.023529411764705882f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const CrownJewel: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.19607843137254902f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const CrownOfThorns: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.23529411764705882f32,
    b: 0.2f32,
    a: 1f32,
};
pub const CrudeBanana: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.7686274509803922f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const CrumblingStatue: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7490196078431373f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const CrunchyCarrot: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.3137254901960784f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const CrusadeKing: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.7647058823529411f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const CrushedIce: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 1.0f32,
    b: 0.9686274509803922f32,
    a: 1f32,
};
pub const CryMeARiver: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.47058823529411764f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const CryOfARose: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.23529411764705882f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const CryoFreeze: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.9254901960784314f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const Crystal: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.8470588235294118f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const CrystalGem: Colour = Colour {
    r: 0.4745098039215686f32,
    g: 0.8156862745098039f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const CrystalLake: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.7098039215686275f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const CrystalPalace: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.8117647058823529f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const CubaLibre: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.2196078431372549f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const Cucumber: Colour = Colour {
    r: 0.0f32,
    g: 0.39215686274509803f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CucumberBomber: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const CucumberMilk: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.9450980392156862f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Cumin: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.5176470588235295f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const Cumulus: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9529411764705882f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Cupid: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.6980392156862745f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const CupidSEye: Colour = Colour {
    r: 1.0f32,
    g: 0.13333333333333333f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Curry: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.6392156862745098f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const CurryBubbles: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.7176470588235294f32,
    b: 0.0f32,
    a: 1f32,
};
pub const CurrySauce: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.6196078431372549f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const Currywurst: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.2f32,
    a: 1f32,
};
pub const CursedBlack: Colour = Colour {
    r: 0.07450980392156863f32,
    g: 0.07450980392156863f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const CuteCrab: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.26666666666666666f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Cuttlefish: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.7333333333333333f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const Cyan: Colour = Colour {
    r: 0.058823529411764705f32,
    g: 0.9411764705882353f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const CyberNeonGreen: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const CyberYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.8313725490196079f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Cyberpink: Colour = Colour {
    r: 1.0f32,
    g: 0.12549019607843137f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Cypress: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.36470588235294116f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const Daffodil: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const DaintyPeach: Colour = Colour {
    r: 1.0f32,
    g: 0.803921568627451f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const DaisyDesi: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8745098039215686f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const DallasDust: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.8784313725490196f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const DampenedBlack: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.2784313725490196f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const DancingSea: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.30196078431372547f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const Dandelion: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8745098039215686f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const Danger: Colour = Colour {
    r: 1.0f32,
    g: 0.054901960784313725f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const DangerousAffair: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.00784313725490196f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const DangerousRobot: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.7725490196078432f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const Dark: Colour = Colour {
    r: 0.10588235294117647f32,
    g: 0.1411764705882353f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const DarkAndStormy: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.24705882352941178f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const DarkAges: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.596078431372549f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const DarkAsNight: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.3215686274509804f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const DarkBlue: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.3568627450980392f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const DarkCharcoal: Colour = Colour {
    r: 0.2f32,
    g: 0.19607843137254902f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const DarkChocolate: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.2901960784313726f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const DarkCrypt: Colour = Colour {
    r: 0.24705882352941178f32,
    g: 0.27058823529411763f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const DarkCyan: Colour = Colour {
    r: 0.0f32,
    g: 0.5450980392156862f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const DarkDenim: Colour = Colour {
    r: 0.0f32,
    g: 0.3333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const DarkDreams: Colour = Colour {
    r: 0.2f32,
    g: 0.13333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const DarkEclipse: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const DarkForest: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.4117647058823529f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const DarkGalaxy: Colour = Colour {
    r: 0.0f32,
    g: 0.09411764705882353f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const DarkKnight: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.09803921568627451f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const DarkMatter: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.00392156862745098f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const DarkMoon: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.09019607843137255f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const DarkOlive: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.24313725490196078f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const DarkOrange: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.3176470588235294f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const DarkOrchestra: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.10588235294117647f32,
    b: 0.09803921568627451f32,
    a: 1f32,
};
pub const DarkPink: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.2549019607843137f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const DarkPrince: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.4235294117647059f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const DarkPurple: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.023529411764705882f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const DarkRed: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const DarkRift: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.043137254901960784f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const DarkRoast: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.17647058823529413f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const DarkRum: Colour = Colour {
    r: 0.27058823529411763f32,
    g: 0.21176470588235294f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const DarkSalmonInjustice: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.5843137254901961f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const DarkSanctuary: Colour = Colour {
    r: 0.24705882352941178f32,
    g: 0.00392156862745098f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const DarkSapphire: Colour = Colour {
    r: 0.03137254901960784f32,
    g: 0.1450980392156863f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const DarkSecret: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.3254901960784314f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const DarkSerpent: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.2f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const DarkSoul: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const DarkSouls: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.6392156862745098f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const DarkSpace: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.2901960784313726f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const DarkVeil: Colour = Colour {
    r: 0.0784313725490196f32,
    g: 0.07450980392156863f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const DarkVoid: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.08235294117647059f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const DarkWood: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.3686274509803922f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const DarkestDungeon: Colour = Colour {
    r: 0.4f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const DarkestForest: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.2f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const DarthVader: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.1450980392156863f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const DayOnMercury: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.8235294117647058f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const DazzlingRed: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.17254901960784313f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const DeadForest: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.29411764705882354f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const DeadPixel: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.22745098039215686f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const DeadlyDepths: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const DeadlyYellow: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.6784313725490196f32,
    b: 0.0f32,
    a: 1f32,
};
pub const DearDarling: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.00392156862745098f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const DearReader: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9529411764705882f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const DeathByChocolate: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.26666666666666666f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const DeathOfAStar: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.3764705882352941f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const DebianRed: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.0392156862745098f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const DecadialPink: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.792156862745098f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const DecreasingBrown: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.4627450980392157f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const DeepForest: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.27450980392156865f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const DeepForestialEscapade: Colour = Colour {
    r: 0.2f32,
    g: 0.3333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const DeepFried: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.6901960784313725f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const DeepFriedSunRays: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.7803921568627451f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const DeepGreen: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.34901960784313724f32,
    b: 0.058823529411764705f32,
    a: 1f32,
};
pub const DeepIndigo: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.33725490196078434f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const DeepLagoon: Colour = Colour {
    r: 0.0f32,
    g: 0.35294117647058826f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const DeepNight: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.2980392156862745f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const DeepPond: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.26666666666666666f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const DeepPoolTeal: Colour = Colour {
    r: 0.21176470588235294f32,
    g: 0.42745098039215684f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const DeepSaffron: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const DeepSeaBase: Colour = Colour {
    r: 0.17254901960784313f32,
    g: 0.17254901960784313f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const DeepSeaDiver: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.3607843137254902f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const DeepSeaNightmare: Colour = Colour {
    r: 0.0f32,
    g: 0.13725490196078433f32,
    b: 0.4f32,
    a: 1f32,
};
pub const DeepSkyBlue: Colour = Colour {
    r: 0.050980392156862744f32,
    g: 0.4588235294117647f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const DeepSpaceRodeo: Colour = Colour {
    r: 0.2f32,
    g: 0.13333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const DeeplyEmbarrassed: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.6980392156862745f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const DeepseaKraken: Colour = Colour {
    r: 0.03137254901960784f32,
    g: 0.1450980392156863f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Deer: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.5294117647058824f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const DelayedYellow: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9764705882352941f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const DeliYellow: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.7098039215686275f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const DelicateCloud: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8745098039215686f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const DelicateIce: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.8235294117647058f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const DelicateLemon: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const DelicateSeashell: Colour = Colour {
    r: 1.0f32,
    g: 0.9372549019607843f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const DélicieuxAuChocolat: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.12549019607843137f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const DelightfulPastry: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9058823529411765f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const DeltaMint: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.9019607843137255f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const DemeterGreen: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.8f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const DemonicKiss: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.16862745098039217f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const Denim: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.2627450980392157f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const DensetsuGreen: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.6f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const DepthsOfNight: Colour = Colour {
    r: 0.17254901960784313f32,
    g: 0.19215686274509805f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Desert: Colour = Colour {
    r: 0.8f32,
    g: 0.6784313725490196f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const DesertDessert: Colour = Colour {
    r: 1.0f32,
    g: 0.7294117647058823f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const DesertDune: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.6705882352941176f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const DesertLocust: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.6431372549019608f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const DesertTemple: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8f32,
    b: 0.6f32,
    a: 1f32,
};
pub const DesertedBeach: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.8588235294117647f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Desirable: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.20392156862745098f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const Desire: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.23529411764705882f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const DesiredDawn: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.8431372549019608f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const DetectiveCoat: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.5254901960784314f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const DevilSAdvocate: Colour = Colour {
    r: 1.0f32,
    g: 0.2f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const DeviledEggs: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.803921568627451f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Devilish: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.2f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const DevilishDiva: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.4666666666666667f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const Diamond: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9686274509803922f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const DiamondCut: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.9137254901960784f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const DiamondWhite: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.9372549019607843f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const Diesel: Colour = Colour {
    r: 0.19607843137254902f32,
    g: 0.17254901960784313f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const DijonMustard: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.792156862745098f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const Dill: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.4666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Dim: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.7607843137254902f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const DinosaurEgg: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7294117647058823f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const DippedInCream: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9647058823529412f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const DireWolf: Colour = Colour {
    r: 0.1568627450980392f32,
    g: 0.1568627450980392f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const DisappearingMemories: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8901960784313725f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const DiscoBall: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.8313725490196079f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const DiscreetOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.6784313725490196f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const DiscretePink: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8588235294117647f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const DistantCloud: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.9176470588235294f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const DistantHomeworld: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.8627450980392157f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const DistantLandscape: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.9372549019607843f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Diva: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.6274509803921569f32,
    b: 1.0f32,
    a: 1f32,
};
pub const DivaMecha: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const DivaPink: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.25882352941176473f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const DiverSEden: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.4745098039215686f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const DivinePleasure: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.9372549019607843f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const DocksideRed: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.20784313725490197f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Doctor: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9764705882352941f32,
    b: 0.9764705882352941f32,
    a: 1f32,
};
pub const Dogwood: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9176470588235294f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const DolcePink: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8509803921568627f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const DollarBill: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.7333333333333333f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const Dolly: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9450980392156862f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const DollyCheek: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.788235294117647f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const Dolphin: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.7686274509803922f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const DonTBeShy: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.17254901960784313f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const DonegalGreen: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.3333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const DönerKebab: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.4666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const DonkeyKong: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.25882352941176473f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const DornYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.9490196078431372f32,
    b: 0.0f32,
    a: 1f32,
};
pub const DoubleCream: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8509803921568627f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const Dove: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.6784313725490196f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const DoveWing: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.8509803921568627f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const DownFeathers: Colour = Colour {
    r: 1.0f32,
    g: 0.9764705882352941f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const DrWho: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.34509803921568627f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const DrWhite: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9803921568627451f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const DragonBall: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const DragonFire: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.2901960784313726f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const DragonFruit: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.34901960784313724f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const DragonSBlood: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.25098039215686274f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const DragonSBreath: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.06274509803921569f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const DragonSGold: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.8784313725490196f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Dragonfly: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.2901960784313726f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const DramaQueen: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.4470588235294118f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const DramaticBlue: Colour = Colour {
    r: 0.1411764705882353f32,
    g: 0.0f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const DreamLand: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.6705882352941176f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const DreamOfSpring: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8117647058823529f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const DreamSetting: Colour = Colour {
    r: 1.0f32,
    g: 0.4666666666666667f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const DreamVapor: Colour = Colour {
    r: 0.8f32,
    g: 0.6f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const DreamlessSleep: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const DreamyCandyForest: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.5843137254901961f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const DriedTomatoes: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.3764705882352941f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const Driftwood: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.47843137254901963f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const DripCoffee: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.1568627450980392f32,
    b: 0.0392156862745098f32,
    a: 1f32,
};
pub const DrippingWisteria: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.6f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Droplet: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.8666666666666667f32,
    b: 1.0f32,
    a: 1f32,
};
pub const DroppedBrick: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.2f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Drover: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9215686274509803f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const DrunktankPink: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const DrunkenFlamingo: Colour = Colour {
    r: 1.0f32,
    g: 0.3333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const DryBone: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8745098039215686f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const DryRose: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.1843137254901961f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const DuckButter: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.7803921568627451f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const DuckHunt: Colour = Colour {
    r: 0.0f32,
    g: 0.34509803921568627f32,
    b: 0.0f32,
    a: 1f32,
};
pub const DucklingFluff: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9882352941176471f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const Dumpling: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8666666666666667f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Dune: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.7529411764705882f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const DungeonKeeper: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.18823529411764706f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const DurianWhite: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8156862745098039f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const Dusk: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.32941176470588235f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const DuskyMood: Colour = Colour {
    r: 0.592156862745098f32,
    g: 0.6078431372549019f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const DustOfTheMoon: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.788235294117647f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const DustStorm: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.8274509803921568f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const DustToDust: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.7372549019607844f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const DustyBoots: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.7529411764705882f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const DustyChimney: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const DustyDuchess: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.5137254901960784f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const DwarfFortress: Colour = Colour {
    r: 0.11372549019607843f32,
    g: 0.00784313725490196f32,
    b: 0.0f32,
    a: 1f32,
};
pub const DwarvenBronze: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.396078431372549f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const DwindlingDandelion: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9137254901960784f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const DyingStormBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Dynamite: Colour = Colour {
    r: 1.0f32,
    g: 0.26666666666666666f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const DynastyGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.596078431372549f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const Eagle: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.4235294117647059f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const EarlGrey: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.592156862745098f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const Earthbound: Colour = Colour {
    r: 0.6431372549019608f32,
    g: 0.5411764705882353f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const Earthworm: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.5058823529411764f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const EasterEgg: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.592156862745098f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const EatYourGreens: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.40784313725490196f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const EbiBrown: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.23529411764705882f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const Ebony: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.2f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const Eclipse: Colour = Colour {
    r: 0.24705882352941178f32,
    g: 0.2235294117647059f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const Ecological: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.4980392156862745f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const EcstaticRed: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Edamame: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.6392156862745098f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const EgaGreen: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 1.0f32,
    b: 0.027450980392156862f32,
    a: 1f32,
};
pub const EggToast: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.788235294117647f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const EggYolk: Colour = Colour {
    r: 1.0f32,
    g: 0.807843137254902f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const Eggnog: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9176470588235294f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const Eggplant: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.0196078431372549f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const Eggshell: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9176470588235294f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const EiffelTower: Colour = Colour {
    r: 0.6f32,
    g: 0.5568627450980392f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const Eigengrau: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.08627450980392157f32,
    b: 0.11372549019607843f32,
    a: 1f32,
};
pub const EightBall: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.0196078431372549f32,
    b: 0.0392156862745098f32,
    a: 1f32,
};
pub const ElasticPink: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.6509803921568628f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const EldenRingOrange: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.5411764705882353f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const Electra: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.7058823529411765f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const ElectricBanana: Colour = Colour {
    r: 0.984313725490196f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const ElectricBlood: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.23921568627450981f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const ElectricEel: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const ElectricLaserLime: Colour = Colour {
    r: 0.14901960784313725f32,
    g: 1.0f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const ElectricLettuce: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.8196078431372549f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const ElectricYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.9882352941176471f32,
    b: 0.0f32,
    a: 1f32,
};
pub const ElectrifyingKiss: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.10980392156862745f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const ElegantPurpleGown: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.13725490196078433f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const ElephantInTheRoom: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.6627450980392157f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const EliteTeal: Colour = Colour {
    r: 0.07450980392156863f32,
    g: 0.2f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const Embarrassed: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4666666666666667f32,
    b: 0.6f32,
    a: 1f32,
};
pub const EmbarrassedFrog: Colour = Colour {
    r: 0.6f32,
    g: 0.4f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Embarrassment: Colour = Colour {
    r: 1.0f32,
    g: 0.4666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Emerald: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.5607843137254902f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const EmeraldBliss: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.7411764705882353f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const EmeraldForest: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.2627450980392157f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const EmeraldGlitter: Colour = Colour {
    r: 0.4f32,
    g: 0.7333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const EmeraldIcePalace: Colour = Colour {
    r: 0.16470588235294117f32,
    g: 0.9607843137254902f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const EmeraldOasis: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.6313725490196078f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const EmeraldRain: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.7843137254901961f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const Emergency: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.09803921568627451f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Eminence: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.2235294117647059f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const EmojiYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.8705882352941177f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const EmperorJade: Colour = Colour {
    r: 0.0f32,
    g: 0.4823529411764706f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const Empress: Colour = Colour {
    r: 0.48627450980392156f32,
    g: 0.44313725490196076f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const Emptiness: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9921568627450981f32,
    b: 0.9882352941176471f32,
    a: 1f32,
};
pub const EnchantedForest: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.5098039215686274f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const EnchantedGlen: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.42745098039215684f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const EnchantedLavender: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.6392156862745098f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const EnchantingIvy: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.34901960784313724f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const EndOfSummer: Colour = Colour {
    r: 0.8f32,
    g: 0.5607843137254902f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const Endive: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.8823529411764706f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const EndlessGalaxy: Colour = Colour {
    r: 0.0f32,
    g: 0.0f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const EndlessHorizon: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.8588235294117647f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const EndlessRiver: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.47843137254901963f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const EndlessSummer: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8117647058823529f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Endo: Colour = Colour {
    r: 0.36470588235294116f32,
    g: 0.6431372549019608f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const EnglishBreakfast: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const EnglishManor: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.5058823529411764f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const EnglishWalnut: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.16862745098039217f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const EngulfedInLight: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9529411764705882f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const Enoki: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9803921568627451f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Enraged: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Envy: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.6470588235294118f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const EosinPink: Colour = Colour {
    r: 1.0f32,
    g: 0.3686274509803922f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const EphemeralBlue: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.8313725490196079f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const EphemeralRed: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.8117647058823529f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const EpicBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.4f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Epink: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.2f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Equanimity: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.6627450980392157f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Equestrienne: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.4588235294117647f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const ErrigalWhite: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9490196078431372f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const Escalope: Colour = Colour {
    r: 0.8f32,
    g: 0.5333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Escargot: Colour = Colour {
    r: 1.0f32,
    g: 0.9450980392156862f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const Espresso: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.19215686274509805f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const EspressoBar: Colour = Colour {
    r: 0.3568627450980392f32,
    g: 0.24705882352941178f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const EspressoCrema: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.611764705882353f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const EspressoMacchiato: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.2784313725490196f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const EspressoMartini: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.22745098039215686f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Estragon: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.6862745098039216f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const EternalFlame: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.24705882352941178f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const EternalSummer: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8980392156862745f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const EternalWinter: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.9803921568627451f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Ether: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.6980392156862745f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const EtherealMoonlight: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.8941176470588236f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const EtherealWoods: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.3686274509803922f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Eucalyptus: Colour = Colour {
    r: 0.19607843137254902f32,
    g: 0.592156862745098f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const EveningGlow: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8431372549019608f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const Everglade: Colour = Colour {
    r: 0.14901960784313725f32,
    g: 0.2627450980392157f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Evergreen: Colour = Colour {
    r: 0.07058823529411765f32,
    g: 0.3568627450980392f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const EverlastingIce: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9921568627450981f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const EvilCigar: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.12549019607843137f32,
    b: 0.0f32,
    a: 1f32,
};
pub const EvilForces: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Excalibur: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.3803921568627451f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const ExclusiveElixir: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9450980392156862f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const ExclusiveIvory: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.8470588235294118f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const ExhilaratingGreen: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.7803921568627451f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const ExitLight: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const ExoticEscape: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.8509803921568627f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const ExoticLilac: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.596078431372549f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const ExplodingStar: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8470588235294118f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const ExplorerOfTheGalaxies: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.12156862745098039f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const ExplosiveGrey: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.7686274509803922f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const ExplosivePurple: Colour = Colour {
    r: 0.8f32,
    g: 0.06666666666666667f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const ExtraFuchsia: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.20392156862745098f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const ExtraLife: Colour = Colour {
    r: 0.41568627450980394f32,
    g: 0.7058823529411765f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const Extraviolet: Colour = Colour {
    r: 0.4f32,
    g: 0.06666666666666667f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const ExtremeCarrot: Colour = Colour {
    r: 1.0f32,
    g: 0.44313725490196076f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Eyeball: Colour = Colour {
    r: 1.0f32,
    g: 0.984313725490196f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const FabricOfLove: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const FabricOfSpace: Colour = Colour {
    r: 0.20392156862745098f32,
    g: 0.09019607843137255f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const FabulousFawn: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.7568627450980392f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const FabulousFuchsia: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const FadedLetter: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.6745098039215687f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const FadingHorizon: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.13333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const FadingLove: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.45098039215686275f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const FadingNight: Colour = Colour {
    r: 0.2f32,
    g: 0.4666666666666667f32,
    b: 0.8f32,
    a: 1f32,
};
pub const FailWhale: Colour = Colour {
    r: 0.6f32,
    g: 0.8f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const FaintGold: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.5803921568627451f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const FairyDust: Colour = Colour {
    r: 1.0f32,
    g: 0.9098039215686274f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const FairyFloss: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.788235294117647f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const FairyTale: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.7058823529411765f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const FairyTaleGreen: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.8f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const FakeBlonde: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9019607843137255f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const FakeJade: Colour = Colour {
    r: 0.07450980392156863f32,
    g: 0.9176470588235294f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const FakeLove: Colour = Colour {
    r: 0.8f32,
    g: 0.4666666666666667f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Falafel: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.4666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const FallenBlossoms: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.6980392156862745f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const FallenPetals: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8784313725490196f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const FanaticFuchsia: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.6f32,
    a: 1f32,
};
pub const FancyFuchsia: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const FancyRedWine: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.01568627450980392f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const Fandango: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.2f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const FantasyRomance: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.22745098039215686f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const FarmerSMarket: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.5686274509803921f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const FatGold: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.7372549019607844f32,
    b: 0.0f32,
    a: 1f32,
};
pub const FatSmooch: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.3254901960784314f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const FatalFury: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.19607843137254902f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const Fatback: Colour = Colour {
    r: 1.0f32,
    g: 0.9686274509803922f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const FattyFuchsia: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const FattySashimi: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7686274509803922f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const Fawn: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.6862745098039216f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const FeastyFuchsia: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Feather: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8509803921568627f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const Featherbed: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.796078431372549f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const FederationOfLove: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.06274509803921569f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const Fedora: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.33725490196078434f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const Feijoa: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.8431372549019608f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const Feminism: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.3411764705882353f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const FemmeFatale: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.5215686274509804f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const FennecFox: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8431372549019608f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const FennelFiasco: Colour = Colour {
    r: 0.0f32,
    g: 0.6666666666666666f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const FennelFiesta: Colour = Colour {
    r: 0.0f32,
    g: 0.7333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const FennelFlower: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.6666666666666666f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Fennelly: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.6196078431372549f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const Fern: Colour = Colour {
    r: 0.32941176470588235f32,
    g: 0.5529411764705883f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Ferntastic: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.6705882352941176f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const Ferocious: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.14901960784313725f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const FerociousFlamingo: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const FerociousFox: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.36470588235294116f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const FerociousFuchsia: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const FertilityGreen: Colour = Colour {
    r: 0.4f32,
    g: 0.9882352941176471f32,
    b: 0.0f32,
    a: 1f32,
};
pub const FestiveBordeaux: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.058823529411764705f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const FestiveFerret: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8745098039215686f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const Feta: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.8784313725490196f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const FeverDream: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.3333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Feverish: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.4f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const FeverishPassion: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.30196078431372547f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const FibonacciBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.13725490196078433f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Ficus: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.34901960784313724f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const FicusElastica: Colour = Colour {
    r: 0.0f32,
    g: 0.3803921568627451f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const FiddleleafFig: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.7843137254901961f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const FierceRed: Colour = Colour {
    r: 0.8f32,
    g: 0.0f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const FieryGlow: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.3254901960784314f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const Fiesta: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8470588235294118f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const FijiSands: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.792156862745098f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const FilmNoir: Colour = Colour {
    r: 0.2784313725490196f32,
    g: 0.2235294117647059f32,
    b: 0.2f32,
    a: 1f32,
};
pub const FinalDeparture: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9607843137254902f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const FinePine: Colour = Colour {
    r: 0.0f32,
    g: 0.5333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const FingerBanana: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.7568627450980392f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const FinnishFiord: Colour = Colour {
    r: 0.36470588235294116f32,
    g: 0.6901960784313725f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const Fiord: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.35294117647058826f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const Fire: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.24705882352941178f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const FireAnt: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.39215686274509803f32,
    b: 0.0f32,
    a: 1f32,
};
pub const FireBolt: Colour = Colour {
    r: 0.8f32,
    g: 0.26666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const FireEngine: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.0f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const FireHydrant: Colour = Colour {
    r: 1.0f32,
    g: 0.050980392156862744f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Fireball: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.12549019607843137f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const Firebug: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.3607843137254902f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Firecracker: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.39215686274509803f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const FireflyGlow: Colour = Colour {
    r: 1.0f32,
    g: 0.9529411764705882f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const Firewatch: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const FirstCrush: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8862745098039215f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const FirstDayOfSummer: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9058823529411765f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const FirstLove: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.4588235294117647f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const FirstSnow: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.9372549019607843f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const FishBone: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.8509803921568627f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const FishBoy: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.8666666666666667f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const FishCeviche: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8823529411764706f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const FishPond: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.7843137254901961f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const FisherKing: Colour = Colour {
    r: 0.3176470588235294f32,
    g: 0.5098039215686274f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const FistOfTheNorthStar: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const FiveStar: Colour = Colour {
    r: 1.0f32,
    g: 0.6666666666666666f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Fizz: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.8588235294117647f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const FizzyPeach: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.7372549019607844f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const Flamboyant: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.23921568627450981f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const FlamboyantFlamingo: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.26666666666666666f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const Flame: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.34509803921568627f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const FlameOfPrometheus: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.23529411764705882f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const FlameSeal: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8862745098039215f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const Flamenco: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.5254901960784314f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const FlamingCauldron: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.6392156862745098f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const FlamingCherry: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.12549019607843137f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const FlamingFlamingo: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.3333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const FlamingHotFlamingoes: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const FlamingJune: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const FlamingOrange: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4f32,
    b: 0.2f32,
    a: 1f32,
};
pub const FlamingoQueen: Colour = Colour {
    r: 0.8f32,
    g: 0.2f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Flan: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8901960784313725f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const FlashInThePan: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Flashlight: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9333333333333333f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const FlatteredFlamingo: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const FleurDeSelCaramel: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.5294117647058824f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const Fleurdelis: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.5647058823529412f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const FlickeringFirefly: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9647058823529412f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const FlickeringLight: Colour = Colour {
    r: 1.0f32,
    g: 0.9450980392156862f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const FlickrPink: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.0f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const FlintRock: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.5803921568627451f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const FlipACoin: Colour = Colour {
    r: 0.8f32,
    g: 0.8666666666666667f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Flipflop: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.7686274509803922f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const Flirt: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.1803921568627451f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const FlirtatiousFlamingo: Colour = Colour {
    r: 0.8f32,
    g: 0.13333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const FlirtyRose: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.3686274509803922f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const FlirtySalmon: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.4392156862745098f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const FloatingFeather: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.8470588235294118f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const Flood: Colour = Colour {
    r: 0.4f32,
    g: 0.4666666666666667f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Flora: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.9803921568627451f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const FloridaSAlligator: Colour = Colour {
    r: 0.4f32,
    g: 0.26666666666666666f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const FluffyDuckling: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8745098039215686f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const Fluorescence: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.8196078431372549f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const FluorescentGreen: Colour = Colour {
    r: 0.03137254901960784f32,
    g: 1.0f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const FluorescentPink: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.0784313725490196f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const FlushOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.43529411764705883f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const FlyAKite: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.8549019607843137f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const FlyAway: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.7019607843137254f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const FlyByNight: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.11764705882352941f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const Flybynight: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.35294117647058826f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const FlyingCarpet: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.4549019607843137f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const FlyingFishBlue: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.2901960784313726f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const Fog: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.8431372549019608f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const FogSyringa: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.7294117647058823f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const FoggyDay: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.8901960784313725f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const FoggyLove: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.7803921568627451f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const FoggyPlateau: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.796078431372549f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const Fogtown: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9411764705882353f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Foil: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.7647058823529411f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const Foliage: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.7019607843137254f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Fondant: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8862745098039215f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const Fondue: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9607843137254902f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const FoolSGold: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.8196078431372549f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const ForbiddenFruit: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.4823529411764706f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const ForbiddenPeanut: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.5019607843137255f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const ForceOfNature: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.807843137254902f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const Forest: Colour = Colour {
    r: 0.043137254901960784f32,
    g: 0.3333333333333333f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const ForestEmpress: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.4392156862745098f32,
    b: 0.08627450980392157f32,
    a: 1f32,
};
pub const Forester: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.6549019607843137f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const Forestial: Colour = Colour {
    r: 0.0f32,
    g: 0.4666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const ForestialOutpost: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.4f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Forestry: Colour = Colour {
    r: 0.1843137254901961f32,
    g: 0.26666666666666666f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const Forgetmenot: Colour = Colour {
    r: 0.0f32,
    g: 0.5294117647058824f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const ForgivenSin: Colour = Colour {
    r: 1.0f32,
    g: 0.06666666666666667f32,
    b: 0.6f32,
    a: 1f32,
};
pub const ForgottenMosque: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.8509803921568627f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const ForgottenSandstone: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.6509803921568628f32,
    b: 0.5882352941176471f32,
    a: 1f32,
};
pub const FortuneCookie: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.7725490196078432f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const FossilStone: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.8666666666666667f32,
    b: 0.8f32,
    a: 1f32,
};
pub const FoundationWhite: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const FourLeafClover: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.5607843137254902f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const Fox: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.3058823529411765f32,
    b: 0.2f32,
    a: 1f32,
};
pub const FoxyPink: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.5843137254901961f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const FrailFuchsia: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Framboise: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.0f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Frankenstein: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.6274509803921569f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const Frappé: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.6823529411764706f32,
    b: 0.6f32,
    a: 1f32,
};
pub const FrappéAuChocolat: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.40784313725490196f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const FreeSpirit: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.9333333333333333f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Freefall: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.3215686274509804f32,
    b: 0.4f32,
    a: 1f32,
};
pub const FreezeUp: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.9137254901960784f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const FreezingVapor: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.9137254901960784f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const FreezyBreezy: Colour = Colour {
    r: 0.6f32,
    g: 0.9333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const FrenchBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.4470588235294118f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const FrenchFry: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.7607843137254902f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const FrenchOak: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.6196078431372549f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const FrenchPorcelain: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9568627450980393f32,
    b: 0.9647058823529412f32,
    a: 1f32,
};
pub const FrenchVanilla: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8823529411764706f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const FrenchWine: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.11764705882352941f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const FrenchWinery: Colour = Colour {
    r: 0.6f32,
    g: 0.06666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Fresco: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8588235294117647f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const FreshAir: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.9058823529411765f32,
    b: 1.0f32,
    a: 1f32,
};
pub const FreshBlueOfBelAir: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.6039215686274509f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const FreshBreeze: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.9294117647058824f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const FreshCutGrass: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.796078431372549f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const FreshGum: Colour = Colour {
    r: 1.0f32,
    g: 0.6666666666666666f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const FreshSnow: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9372549019607843f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const FreshlyBaked: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.7568627450980392f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const FreshlyPurpleized: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.3137254901960784f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const FreshlyRoastedCoffee: Colour = Colour {
    r: 0.4f32,
    g: 0.2f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Fricassée: Colour = Colour {
    r: 1.0f32,
    g: 0.9019607843137255f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const FriendlyFrost: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.984313725490196f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Frog: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.7372549019607844f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const FrogOnALog: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.7254901960784313f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const FrogPond: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.7137254901960784f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const FrogPrince: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8431372549019608f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const Frogger: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.8392156862745098f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const FroggyPond: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.7294117647058823f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const Frost: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8941176470588236f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const FrostFairy: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8117647058823529f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const Frostbite: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 1.0f32,
    b: 0.9882352941176471f32,
    a: 1f32,
};
pub const FrostedBlueberries: Colour = Colour {
    r: 0.0f32,
    g: 0.3333333333333333f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const FrostedMintHills: Colour = Colour {
    r: 0.8f32,
    g: 1.0f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const FrothyMilk: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9294117647058824f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const FrozenBoubble: Colour = Colour {
    r: 0.0f32,
    g: 0.9333333333333333f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const FrozenCivilization: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.9607843137254902f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const FrozenForest: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.9098039215686274f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const FrozenLandscape: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.8941176470588236f32,
    b: 1.0f32,
    a: 1f32,
};
pub const FrozenMammoth: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8509803921568627f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const FrozenPeriwinkle: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.8196078431372549f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const FrozenSalmon: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.6627450980392157f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const FrozenTomato: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.3333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const FrozenTurquoise: Colour = Colour {
    r: 0.3254901960784314f32,
    g: 0.9647058823529412f32,
    b: 1.0f32,
    a: 1f32,
};
pub const FrozenWave: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.6745098039215687f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const FruitOfPassion: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.4117647058823529f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const FruityLicious: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.5647058823529412f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const Fuchsia: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.050980392156862744f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const FuchsiaFelicity: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.2784313725490196f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const FuchsiaFever: Colour = Colour {
    r: 1.0f32,
    g: 0.3333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const FuchsiaNebula: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const FuchsiaPheromone: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.2980392156862745f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const Fuego: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const FugitiveFlamingo: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const FujiPeak: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9333333333333333f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const FullMoon: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.9529411764705882f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const FunkiPorcini: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6f32,
    b: 0.6f32,
    a: 1f32,
};
pub const FunkyFrog: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.7411764705882353f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const FuriousFox: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.3333333333333333f32,
    b: 0.09803921568627451f32,
    a: 1f32,
};
pub const FuriousFrog: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const FuriousFuchsia: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const FuriousRed: Colour = Colour {
    r: 1.0f32,
    g: 0.06666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const FuriousTiger: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.34509803921568627f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const Furnace: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.2549019607843137f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const FurryLion: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.5764705882352941f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const Fusilli: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9098039215686274f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const FusionRed: Colour = Colour {
    r: 1.0f32,
    g: 0.3803921568627451f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Futon: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9647058823529412f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const FutureFuchsia: Colour = Colour {
    r: 1.0f32,
    g: 0.12549019607843137f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const FuzzyDuckling: Colour = Colour {
    r: 1.0f32,
    g: 0.9176470588235294f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const FuzzySheep: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9137254901960784f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const FuzzyWuzzy: Colour = Colour {
    r: 0.8f32,
    g: 0.4f32,
    b: 0.4f32,
    a: 1f32,
};
pub const GalacticCivilization: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.13333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const GalacticCruise: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const GalacticFederation: Colour = Colour {
    r: 0.2f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const GalacticHighway: Colour = Colour {
    r: 0.2f32,
    g: 0.06666666666666667f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const GalacticPurple: Colour = Colour {
    r: 0.2784313725490196f32,
    g: 0.1803921568627451f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const Galaxea: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.18823529411764706f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const GalaxyBlue: Colour = Colour {
    r: 0.17647058823529413f32,
    g: 0.3215686274509804f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const GalaxyExpress: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.26666666666666666f32,
    b: 0.6f32,
    a: 1f32,
};
pub const GaleOfTheWind: Colour = Colour {
    r: 0.0f32,
    g: 0.47058823529411764f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const GallantGreen: Colour = Colour {
    r: 0.6f32,
    g: 0.6666666666666666f32,
    b: 0.4f32,
    a: 1f32,
};
pub const GameboyScreen: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.6745098039215687f32,
    b: 0.058823529411764705f32,
    a: 1f32,
};
pub const GangstersGold: Colour = Colour {
    r: 1.0f32,
    g: 0.8666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const GardenGoddess: Colour = Colour {
    r: 0.6f32,
    g: 0.807843137254902f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const GardenSnail: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.6941176470588235f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const GardenWeed: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.43137254901960786f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const Gardenia: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9098039215686274f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Garfield: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.32941176470588235f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const Gargoyle: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.7019607843137254f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const GarlicButter: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8745098039215686f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const GarlicClove: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.8431372549019608f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const GarlicToast: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8666666666666667f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const GatewayGrey: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.6274509803921569f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const GatsbyGlitter: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8392156862745098f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const GauntletGrey: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.45098039215686275f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const Gazelle: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.49411764705882355f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const Gecko: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.5686274509803921f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const GeckoSDream: Colour = Colour {
    r: 0.4f32,
    g: 0.6f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Genie: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.2627450980392157f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const GentianBlue: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.13333333333333333f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const GentleCaress: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8431372549019608f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const GentleCold: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.9254901960784314f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const GentleFrost: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.8784313725490196f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const GentleGlow: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8980392156862745f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const GentleSky: Colour = Colour {
    r: 0.6f32,
    g: 0.7411764705882353f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const GeorgiaPeach: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.4470588235294118f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const GermanHop: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.6745098039215687f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const GettingWet: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.8549019607843137f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const Ghost: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.7490196078431373f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const GhostLichen: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.9294117647058824f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const GhostPepper: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.00392156862745098f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const GhostWhisperer: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.8196078431372549f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const GhostWhite: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9725490196078431f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Ghosted: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.8784313725490196f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const Ghoul: Colour = Colour {
    r: 0.4f32,
    g: 0.4666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Giggle: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9411764705882353f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const Gin: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.8745098039215686f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const GinFizz: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9176470588235294f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const GinTonic: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.9215686274509803f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const Ginger: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.396078431372549f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GingerAle: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.6588235294117647f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const GingerBeer: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.4980392156862745f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const GingerDough: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.42745098039215684f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const GingerLemonTea: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const GingerScent: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.5607843137254902f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const Gingerbread: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.2901960784313726f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const GingerbreadCrumble: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.3686274509803922f32,
    b: 0.2f32,
    a: 1f32,
};
pub const GingerbreadHouse: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.6f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Giraffe: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.996078431372549f32,
    b: 0.2f32,
    a: 1f32,
};
pub const GirlPower: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.6078431372549019f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const Girlie: Colour = Colour {
    r: 1.0f32,
    g: 0.8274509803921568f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const GlacialIce: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9137254901960784f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Glacier: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.6941176470588235f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const GlamourPink: Colour = Colour {
    r: 1.0f32,
    g: 0.11372549019607843f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const GlamourWhite: Colour = Colour {
    r: 1.0f32,
    g: 0.9882352941176471f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const Glassmith: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.7098039215686275f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const GlaucousGreen: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.9098039215686274f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const GlazedChestnut: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.4470588235294118f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const GlazedSugar: Colour = Colour {
    r: 1.0f32,
    g: 0.8627450980392157f32,
    b: 0.8f32,
    a: 1f32,
};
pub const GlimpseIntoSpace: Colour = Colour {
    r: 0.07058823529411765f32,
    g: 0.07058823529411765f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const GlimpseOfVoid: Colour = Colour {
    r: 0.2f32,
    g: 0.3333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const GlisteningDawn: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.7294117647058823f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const GlitterIsNotGold: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8627450980392157f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const GlitterLake: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.7333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const GlitterShower: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 1.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const GloriousGreenGlitter: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.9333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const GloriousSunset: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.5215686274509804f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const GlossyBlack: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const GlowInTheDark: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.9921568627450981f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const GlowWorm: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.8352941176470589f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const GlowingLantern: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.7176470588235294f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const GlowingMeteor: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.26666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Glowlight: Colour = Colour {
    r: 1.0f32,
    g: 0.9647058823529412f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const GoGoGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.5411764705882353f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const GoToHellBlack: Colour = Colour {
    r: 0.20392156862745098f32,
    g: 0.17254901960784313f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const GoblinGreen: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 1.0f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const GodOfRain: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.4f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Goddess: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.8823529411764706f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const Godzilla: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.30196078431372547f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const GokuOrange: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.5137254901960784f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const Gold: Colour = Colour {
    r: 1.0f32,
    g: 0.8431372549019608f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GoldButtercup: Colour = Colour {
    r: 1.0f32,
    g: 0.9098039215686274f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const GoldDigger: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.6901960784313725f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const GoldGrillz: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.8784313725490196f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const GoldRush: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.6549019607843137f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const GoldTooth: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.7058823529411765f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const GoldVein: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.7254901960784313f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const GoldWinged: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8392156862745098f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const GoldenBlood: Colour = Colour {
    r: 1.0f32,
    g: 0.06666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const GoldenBoy: Colour = Colour {
    r: 1.0f32,
    g: 0.8666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const GoldenChurro: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.807843137254902f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const GoldenCoin: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8509803921568627f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const GoldenFrame: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.7019607843137254f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const GoldenGinkgo: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9607843137254902f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const GoldenGlitterStorm: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8431372549019608f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const GoldenHarvest: Colour = Colour {
    r: 0.8f32,
    g: 0.8f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const GoldenHour: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.7058823529411765f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const GoldenKingdom: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.7843137254901961f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const GoldenLion: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.792156862745098f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const GoldenMist: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.788235294117647f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const GoldenNugget: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.5568627450980392f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const GoldenOpportunity: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.7529411764705882f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const GoldenPeriod: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8588235294117647f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const GoldenRelic: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.807843137254902f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const GoldenRetriever: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8705882352941177f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const GoldenRod: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.6823529411764706f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const GoldenSpell: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const GoldenSprinkles: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8235294117647058f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const GoldenTalisman: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.7843137254901961f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Goldfinch: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8941176470588236f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const Goldfinger: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Goldfish: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.6784313725490196f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const Goldie: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.615686274509804f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const Goldilocks: Colour = Colour {
    r: 1.0f32,
    g: 0.9529411764705882f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const Golem: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.43137254901960786f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const GolfCourse: Colour = Colour {
    r: 0.35294117647058826f32,
    g: 0.6196078431372549f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const GoodKarma: Colour = Colour {
    r: 0.2f32,
    g: 0.23529411764705882f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const GoodMorning: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9882352941176471f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const GoodNight: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.33725490196078434f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const GooseBill: Colour = Colour {
    r: 1.0f32,
    g: 0.7294117647058823f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const GoryRed: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.03137254901960784f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Gossip: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.8274509803921568f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const Gotham: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.47058823529411764f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const Gothic: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.5333333333333333f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const GothicGold: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.5215686274509804f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const GourmetHoney: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.796078431372549f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const GrainOfSalt: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.8588235294117647f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const GrampsShoehorn: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.5372549019607843f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const GrandBleu: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.32941176470588235f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const GrandCasinoGold: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.803921568627451f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const GrandSunset: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.5529411764705883f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const GrandmaSCameo: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9058823529411765f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const GrandmaSPinkTiles: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.7215686274509804f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const Granite: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.41568627450980394f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const Granola: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.807843137254902f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const Grape: Colour = Colour {
    r: 0.4235294117647059f32,
    g: 0.20392156862745098f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const GrapeCandy: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.3215686274509804f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const GrapeFizz: Colour = Colour {
    r: 0.39215686274509803f32,
    g: 0.2627450980392157f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const GrapeGreen: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.8941176470588236f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const GrapeKiss: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.2784313725490196f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const GrapeRiot: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.27450980392156865f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const GrapeTaffy: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8549019607843137f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const Grapefruit: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.34901960784313724f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const GrapesOfItaly: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.2901960784313726f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const Grapest: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Graphite: Colour = Colour {
    r: 0.2196078431372549f32,
    g: 0.20392156862745098f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const GraphiteBlack: Colour = Colour {
    r: 0.14901960784313725f32,
    g: 0.16470588235294117f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const Grass: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.6745098039215687f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const Grasshopper: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.5098039215686274f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const GratinDauphinois: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.8235294117647058f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const Grauzone: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.6392156862745098f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const Gravlax: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.5137254901960784f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const GreasyGreens: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.4666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const GreatVoid: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.3411764705882353f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const GreedyGecko: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.6f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const GreekGoddess: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9137254901960784f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const GreekOlive: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.5254901960784314f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const Green: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GreenBellPepper: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GreenCommando: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.5019607843137255f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const GreenEnvy: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GreenField: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const GreenGlimmer: Colour = Colour {
    r: 0.0f32,
    g: 0.7333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GreenGlint: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.9450980392156862f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const GreenGoblin: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const GreenGoddess: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.6784313725490196f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const GreenMana: Colour = Colour {
    r: 0.14901960784313725f32,
    g: 0.7058823529411765f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const GreenNotFound: Colour = Colour {
    r: 0.25098039215686274f32,
    g: 0.26666666666666666f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const GreenOlive: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.5450980392156862f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const GreenPriestess: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.8666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const GreenRelict: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.5294117647058824f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const GreenRevolution: Colour = Colour {
    r: 0.0f32,
    g: 0.6f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const GreenScreen: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GreenSymphony: Colour = Colour {
    r: 0.4f32,
    g: 0.6666666666666666f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const GreenTea: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.7137254901960784f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const GreenTeaMochi: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.6627450980392157f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const GreenThumb: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.6f32,
    b: 0.0f32,
    a: 1f32,
};
pub const GreenVenom: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.9725490196078431f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const GreenWithEnvy: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Greenfinch: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.6627450980392157f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const Greenhorn: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.8f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const Greenhouse: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.38823529411764707f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Greenivorous: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.8784313725490196f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const Gremlin: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.6f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const GrenadinePink: Colour = Colour {
    r: 1.0f32,
    g: 0.3803921568627451f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const Grey: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.5019607843137255f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const GreyArea: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.5764705882352941f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const GreyMarble: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.7058823529411765f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const GreySheep: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.6666666666666666f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const GreyWeb: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.4f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const Greybeard: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.8156862745098039f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const Greystone: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.7254901960784313f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const Griffin: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.5215686274509804f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const Grilled: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.24705882352941178f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const GrilledTomato: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.20784313725490197f32,
    b: 0.09803921568627451f32,
    a: 1f32,
};
pub const GrimGrey: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.8627450980392157f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const GrimWhite: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9450980392156862f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const Grisaille: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.592156862745098f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const Grizzly: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.34509803921568627f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const GroovyGiraffe: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const GroovyLemonPie: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.7450980392156863f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const GrotesqueGreen: Colour = Colour {
    r: 0.39215686274509803f32,
    g: 0.9137254901960784f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const GruyèreCheese: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.8705882352941177f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Guacamole: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.596078431372549f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const GuardianOfGardens: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const GuineaPig: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.4627450980392157f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const Gull: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.5490196078431373f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const GumLeaf: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.788235294117647f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const GummyDolphins: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.6627450980392157f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const GunPowder: Colour = Colour {
    r: 0.2823529411764706f32,
    g: 0.2784313725490196f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const Gunmetal: Colour = Colour {
    r: 0.3254901960784314f32,
    g: 0.3843137254901961f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const GunsNRoses: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Gunsmoke: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.48627450980392156f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const GypsyDancer: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.48627450980392156f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const H2O: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.8823529411764706f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Habañero: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.5215686274509804f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const HabañeroGold: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8117647058823529f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const HaddockSSweater: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.47843137254901963f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const HadfieldBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.4666666666666667f32,
    b: 1.0f32,
    a: 1f32,
};
pub const HairyHeath: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.20784313725490197f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const Haiti: Colour = Colour {
    r: 0.17254901960784313f32,
    g: 0.16470588235294117f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const Halloween: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.396078431372549f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const HalloweenPunch: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Halo: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.7647058823529411f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const HaltRed: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const HammamBlue: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.8627450980392157f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const HamsterFur: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.5058823529411764f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const HamtaroBrown: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.4549019607843137f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const HanamiPink: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.6705882352941176f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const Handmade: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.45098039215686275f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const HangingGardensOfBabylon: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const HappyCement: Colour = Colour {
    r: 0.592156862745098f32,
    g: 0.6196078431372549f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const HappyHearts: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.38823529411764707f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const HappyHippo: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.5215686274509804f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const HappyPiglets: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.796078431372549f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const HappySkeleton: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9333333333333333f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const HarborMist: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const HarbourBlue: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.4549019607843137f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const HardCandy: Colour = Colour {
    r: 1.0f32,
    g: 0.7333333333333333f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const HaremSilk: Colour = Colour {
    r: 0.0f32,
    g: 0.38823529411764707f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const HarlockSCape: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HarvestAtDusk: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.5254901960784314f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const HarvestGold: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.7176470588235294f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const HarvestTime: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.5294117647058824f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const HatobaPigeon: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.5215686274509804f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const HauntedCandelabra: Colour = Colour {
    r: 0.3411764705882353f32,
    g: 0.26666666666666666f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const HauntedForest: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.1803921568627451f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const HauntedHills: Colour = Colour {
    r: 0.0f32,
    g: 0.2f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const HauntedPurple: Colour = Colour {
    r: 0.6f32,
    g: 0.06666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const HauteCouture: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.1450980392156863f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const Havana: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.16862745098039217f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const HawaiiMorning: Colour = Colour {
    r: 0.0f32,
    g: 0.7333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const HawaiianRaspberry: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const HayDay: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.803921568627451f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const Haystacks: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.6745098039215687f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const Hazel: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.4196078431372549f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Hazelnut: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.44313725490196076f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const HazelnutChocolate: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.24705882352941178f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Hazelwood: Colour = Colour {
    r: 1.0f32,
    g: 0.9529411764705882f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const HazyMoon: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.8627450980392157f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const HeadInTheClouds: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.8666666666666667f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const HeartOfIce: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9882352941176471f32,
    b: 1.0f32,
    a: 1f32,
};
pub const HeartPotion: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.4980392156862745f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const Heartbeat: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Heartless: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.23137254901960785f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const Heartwarming: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.09411764705882353f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const HeatSignature: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.0f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const HeatWave: Colour = Colour {
    r: 1.0f32,
    g: 0.47843137254901963f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HeatherBerry: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.32941176470588235f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const HeavenGates: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.9450980392156862f32,
    b: 1.0f32,
    a: 1f32,
};
pub const HeavenlySky: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.5647058823529412f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const HeavyBrown: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.3843137254901961f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const HeavyCharcoal: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.3254901960784314f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const HeavyCream: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8666666666666667f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const HeavyGreen: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.34509803921568627f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const HeavyHeart: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const HeavyMetal: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.2784313725490196f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const HeavyOrange: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.2627450980392157f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const HeavyRed: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.07058823529411765f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const HeavyViolet: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.33725490196078434f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const HedgeGarden: Colour = Colour {
    r: 0.0f32,
    g: 0.6666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Helium: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8980392156862745f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const HelloDarknessMyOldFriend: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.13333333333333333f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const HelloFall: Colour = Colour {
    r: 0.6f32,
    g: 0.3333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const HelloSpring: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.8666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const HelloSummer: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.7333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const HelloWinter: Colour = Colour {
    r: 0.6f32,
    g: 1.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const HelvetiaRed: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Hemp: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.49019607843137253f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const HerFierceness: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.07058823529411765f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const HerHighness: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.1803921568627451f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const HerMajesty: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.6431372549019608f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const HerVelour: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.37254901960784315f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const Herbal: Colour = Colour {
    r: 0.1607843137254902f32,
    g: 0.6705882352941176f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const HerbalVapors: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 1.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const HerbalWhispers: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.42745098039215684f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Herbalist: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.6196078431372549f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const HerbalistSGarden: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.6f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Herbivore: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const HereComesTheSun: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8745098039215686f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const HeroicRed: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.09803921568627451f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const Heron: Colour = Colour {
    r: 0.41568627450980394f32,
    g: 0.40784313725490196f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const HerringSilver: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.7843137254901961f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const HeyBlue: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.9725490196078431f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Hibernation: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.3176470588235294f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Hibiscus: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.19215686274509805f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Hickory: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.6352941176470588f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const HiddenParadise: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.5450980392156862f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const HiddenSeaGlass: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.8196078431372549f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const HighBlue: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.6588235294117647f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const HighDive: Colour = Colour {
    r: 0.34901960784313724f32,
    g: 0.7254901960784313f32,
    b: 0.8f32,
    a: 1f32,
};
pub const HighGrass: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HighSeas: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.6705882352941176f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const HighSierra: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.8705882352941177f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const HighTide: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.6509803921568628f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const HighVoltage: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 1.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Highland: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.5803921568627451f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const Highlander: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.3254901960784314f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const Highlands: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.5647058823529412f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const HighwayToHell: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.06666666666666667f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const Himalaya: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.38823529411764707f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const HimalayanSalt: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.4666666666666667f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const HinduLotus: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.30196078431372547f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const HintOfBlue: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.8823529411764706f32,
    b: 0.9490196078431372f32,
    a: 1f32,
};
pub const HintOfGreen: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.9176470588235294f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const HintOfMint: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.9450980392156862f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const HintOfOrange: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9019607843137255f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const HintOfPink: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.8941176470588236f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const HintOfRed: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8745098039215686f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const HintOfYellow: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9450980392156862f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Hinterland: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.4235294117647059f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const HippieTrail: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.6666666666666666f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const Hippy: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8980392156862745f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const HipsterHippo: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.7019607843137254f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const HipsterSalmon: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.48627450980392156f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const Hive: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Hobgoblin: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.6784313725490196f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const HokkaidoLavender: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.21176470588235294f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const HoldYourHorses: Colour = Colour {
    r: 0.4392156862745098f32,
    g: 0.32941176470588235f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const HoleInOne: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.6823529411764706f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const HollandTulip: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.596078431372549f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Hollandaise: Colour = Colour {
    r: 1.0f32,
    g: 0.9333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const HollowKnight: Colour = Colour {
    r: 0.2f32,
    g: 0.0f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const HolyCannoli: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.47058823529411764f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const HolyCrow: Colour = Colour {
    r: 0.2f32,
    g: 0.1843137254901961f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const HolyGhost: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9137254901960784f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const HolyGrail: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8431372549019608f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const HomeBrew: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.4823529411764706f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Homegrown: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.5333333333333333f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const HomeopathicBlue: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.9058823529411765f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const HomeopathicGreen: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.9215686274509803f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const HomeopathicLavender: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8784313725490196f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const HomeopathicLilac: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8784313725490196f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const HomeopathicLime: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.9647058823529412f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const HomeopathicMint: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.9176470588235294f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const HomeopathicOrange: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9019607843137255f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const HomeopathicRed: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.8588235294117647f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const HomeopathicRose: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8588235294117647f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const HomeopathicYellow: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9058823529411765f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const Homeworld: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.6f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Honey: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.5372549019607843f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const HoneyAndThyme: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.6666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HoneyBee: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8745098039215686f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const HoneyBunny: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.7215686274509804f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const HoneyCrisp: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.7568627450980392f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const HoneyDo: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9294117647058824f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const HoneyGlow: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.7058823529411765f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const HoneyGold: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.7137254901960784f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const HoneyTeriyaki: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Honeycomb: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const HoneycombGlow: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.8117647058823529f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Honeydew: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 1.0f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const HoneydewSand: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.807843137254902f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const HoneyedGlow: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.7686274509803922f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Honeysuckle: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.9294117647058824f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const HongKongMist: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.5568627450980392f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const HongKongTaxi: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.06274509803921569f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const HoniedWhite: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9372549019607843f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const HonoluluBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.4980392156862745f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Horizon: Colour = Colour {
    r: 0.39215686274509803f32,
    g: 0.5333333333333333f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const HornetSting: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const HorrorSnob: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.30196078431372547f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const Horseradish: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8745098039215686f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const HotBeach: Colour = Colour {
    r: 1.0f32,
    g: 0.9647058823529412f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const HotBrown: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.25882352941176473f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const HotButter: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.615686274509804f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HotCacao: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.4117647058823529f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const HotCaramel: Colour = Colour {
    r: 0.8f32,
    g: 0.43137254901960786f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const HotChilli: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.3176470588235294f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const HotChocolate: Colour = Colour {
    r: 0.40784313725490196f32,
    g: 0.2235294117647059f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const HotCuba: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const HotCurry: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.3568627450980392f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const HotFlaminChilli: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.09411764705882353f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const HotFlamingo: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.34901960784313724f32,
    b: 0.4f32,
    a: 1f32,
};
pub const HotFudge: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.1607843137254902f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const HotJazz: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.18823529411764706f32,
    b: 0.2f32,
    a: 1f32,
};
pub const HotLava: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const HotLips: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.19215686274509805f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const HotMagenta: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const HotPink: Colour = Colour {
    r: 1.0f32,
    g: 0.00784313725490196f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const HotSand: Colour = Colour {
    r: 0.8f32,
    g: 0.6666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HotSauce: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.30980392156862746f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const HotShot: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.30980392156862746f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const Hotspot: Colour = Colour {
    r: 1.0f32,
    g: 0.26666666666666666f32,
    b: 0.2f32,
    a: 1f32,
};
pub const HotterButter: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.5411764705882353f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HotterThanHell: Colour = Colour {
    r: 1.0f32,
    g: 0.26666666666666666f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const HottestOfPinks: Colour = Colour {
    r: 1.0f32,
    g: 0.5019607843137255f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Hourglass: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8784313725490196f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const HowlingPink: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.027450980392156862f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const Hulk: Colour = Colour {
    r: 0.0f32,
    g: 0.5019607843137255f32,
    b: 0.0f32,
    a: 1f32,
};
pub const HumbleBlush: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.803921568627451f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const HumbleHippo: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.6666666666666666f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Hummus: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Hunter: Colour = Colour {
    r: 0.2f32,
    g: 0.3254901960784314f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Hurricane: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.49411764705882355f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Hydra: Colour = Colour {
    r: 0.0f32,
    g: 0.4117647058823529f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const Hydro: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.4549019607843137f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const HyperBlue: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.37254901960784315f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const HyperLightDrifter: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8588235294117647f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const HyperPink: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.0f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const HyperlinkBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const HyperpopGreen: Colour = Colour {
    r: 0.09019607843137255f32,
    g: 0.9764705882352941f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const HypnoticGreen: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.9019607843137255f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const HypnoticRed: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.050980392156862744f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const ILoveYouPink: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.49019607843137253f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const Ibis: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.7019607843137254f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const Ice: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 1.0f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const IceCitadel: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.9725490196078431f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const IceClimber: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.8862745098039215f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const IceCold: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.9176470588235294f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const IceCube: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.8901960784313725f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const IceDagger: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.8980392156862745f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const IceDesert: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.8627450980392157f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const IceIce: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.9215686274509803f32,
    b: 0.6823529411764706f32,
    a: 1f32,
};
pub const IceIceBaby: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const IceTemple: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 1.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Iceberg: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8941176470588236f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Icebreaker: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.7607843137254902f32,
    b: 0.8f32,
    a: 1f32,
};
pub const IcedCoffee: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.5372549019607843f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const IcelandicWinter: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.9058823529411765f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const IcyPink: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.807843137254902f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const IcyPlains: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.8549019607843137f32,
    b: 0.984313725490196f32,
    a: 1f32,
};
pub const Igniting: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8392156862745098f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const Iguana: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.5294117647058824f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const IllicitDarkness: Colour = Colour {
    r: 0.0f32,
    g: 0.00784313725490196f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const IllicitGreen: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.9882352941176471f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const IllicitPink: Colour = Colour {
    r: 1.0f32,
    g: 0.3607843137254902f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const IllicitPurple: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.4666666666666667f32,
    b: 0.9647058823529412f32,
    a: 1f32,
};
pub const IlluminatiGreen: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.5686274509803921f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const Illuminating: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Imperial: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.1843137254901961f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const ImperialIvory: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9098039215686274f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const ImperialPurple: Colour = Colour {
    r: 0.3568627450980392f32,
    g: 0.19215686274509805f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const ImperialRed: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.1607843137254902f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const ImperialYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.6980392156862745f32,
    b: 0.0f32,
    a: 1f32,
};
pub const InAPickle: Colour = Colour {
    r: 0.592156862745098f32,
    g: 0.5490196078431373f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const InForAPenny: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const InGoodTaste: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.8313725490196079f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const InTheDark: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.23529411764705882f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const InThePink: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.7686274509803922f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const InTheRed: Colour = Colour {
    r: 1.0f32,
    g: 0.13333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const InTheShadows: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.7686274509803922f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const InTheTwilight: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.5137254901960784f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const InTheVines: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.27058823529411763f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const IncaGold: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.42745098039215684f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const IncaYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.8274509803921568f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const Incense: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.6039215686274509f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const Inchworm: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.9254901960784314f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const Incision: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const IncubationRed: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.11372549019607843f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const IndianMesa: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.6313725490196078f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const IndianPaleAle: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.7372549019607844f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const IndianRed: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.054901960784313725f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const IndianSaffron: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 0.2f32,
    a: 1f32,
};
pub const IndianSilk: Colour = Colour {
    r: 0.5411764705882353f32,
    g: 0.3411764705882353f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const Indica: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.5490196078431373f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const Indigo: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.0f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Indochine: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.3568627450980392f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const IndocileTiger: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.4196078431372549f32,
    b: 0.0f32,
    a: 1f32,
};
pub const InescapableLover: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.054901960784313725f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const InfectiousLove: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const InfernoOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.26666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const InfiniteNight: Colour = Colour {
    r: 0.027450980392156862f32,
    g: 0.06274509803921569f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const Infinity: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.1568627450980392f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const InfinityAndBeyond: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.49411764705882355f32,
    b: 0.6f32,
    a: 1f32,
};
pub const InfinityPool: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.8313725490196079f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const Infrared: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.2823529411764706f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Infusion: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.8156862745098039f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const InkBlack: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.12549019607843137f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const Inkblot: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.24705882352941178f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Inkjet: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.3333333333333333f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const InlandWaters: Colour = Colour {
    r: 0.48627450980392156f32,
    g: 0.5764705882352941f32,
    b: 0.615686274509804f32,
    a: 1f32,
};
pub const InnocentSnowdrop: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.7803921568627451f32,
    b: 1.0f32,
    a: 1f32,
};
pub const InsomniacBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const InstantNoodles: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8313725490196079f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const IntensePassion: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.19215686274509805f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Intergalactic: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.3176470588235294f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const IntergalacticCowboy: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const IntergalacticHighway: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.19607843137254902f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const IntergalacticSettlement: Colour = Colour {
    r: 0.3568627450980392f32,
    g: 0.11764705882352941f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const InternationalKleinBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.1843137254901961f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const InterstellarBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.06666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const IntimateJournal: Colour = Colour {
    r: 0.8f32,
    g: 0.7333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const IntoTheBlue: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.4823529411764706f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const IntoTheGreen: Colour = Colour {
    r: 0.050980392156862744f32,
    g: 0.4235294117647059f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const IntoTheNight: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.21176470588235294f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const IntoTheStratosphere: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.3215686274509804f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const Intoxicate: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.7333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const IntrigueRed: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.27450980392156865f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const InuitBlue: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.8941176470588236f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Iridescent: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.3568627450980392f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const IrishClover: Colour = Colour {
    r: 0.3254901960784314f32,
    g: 0.45098039215686275f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const IrishCoffee: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.25882352941176473f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const IrishMoor: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.7529411764705882f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const IrishSpring: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.8f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const Iron: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.3686274509803922f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const IronFist: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.803921568627451f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const IronMaiden: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.8196078431372549f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const Ironside: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.5019607843137255f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const IslamicGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.6f32,
    b: 0.0f32,
    a: 1f32,
};
pub const IslandCoral: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.5294117647058824f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const IsleOfDreams: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.8f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const Isolation: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.30196078431372547f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const ItSAGirl: Colour = Colour {
    r: 1.0f32,
    g: 0.8549019607843137f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const ItalianBasil: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.4117647058823529f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const ItalianGrape: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.23921568627450981f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const ItalianRoast: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Ivory: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const IvoryBuff: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8509803921568627f32,
    b: 0.6f32,
    a: 1f32,
};
pub const IvoryTower: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9529411764705882f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const IvoryWedding: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9294117647058824f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const Ivy: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.4823529411764706f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const IvyTopiary: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.3803921568627451f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const JackAndCoke: Colour = Colour {
    r: 0.5725490196078431f32,
    g: 0.058823529411764705f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const Jacko: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.6f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const Jackpot: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.5803921568627451f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Jacuzzi: Colour = Colour {
    r: 0.0f32,
    g: 0.48627450980392156f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const Jade: Colour = Colour {
    r: 0.0f32,
    g: 0.6588235294117647f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const JadeJewel: Colour = Colour {
    r: 0.1411764705882353f32,
    g: 0.49411764705882355f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const JadeSea: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.8784313725490196f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const Jaffa: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.4745098039215686f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const Jaguar: Colour = Colour {
    r: 0.1607843137254902f32,
    g: 0.1607843137254902f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const Jakarta: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8666666666666667f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const Jalapeño: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.5529411764705883f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const JalapeñoRed: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.06666666666666667f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const Jambalaya: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.7098039215686275f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const JamesBlonde: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8901960784313725f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const JapaneseBonsai: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.6235294117647059f32,
    b: 0.5882352941176471f32,
    a: 1f32,
};
pub const Jasmine: Colour = Colour {
    r: 1.0f32,
    g: 0.9568627450980393f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Java: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.592156862745098f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const Jazz: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.17254901960784313f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const JazzyJade: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.8f32,
    a: 1f32,
};
pub const JealousJellyfish: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.6f32,
    a: 1f32,
};
pub const JediNight: Colour = Colour {
    r: 0.01568627450980392f32,
    g: 0.06666666666666667f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const JellyBerry: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const JellySlug: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.4f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const JellyfishSting: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const JetBlack: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.2f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const JetDEau: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.9176470588235294f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const Jewel: Colour = Colour {
    r: 0.07450980392156863f32,
    g: 0.40784313725490196f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const JewelBeetle: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.788235294117647f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const JewelWeed: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.6549019607843137f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const Jigglypuff: Colour = Colour {
    r: 1.0f32,
    g: 0.6666666666666666f32,
    b: 1.0f32,
    a: 1f32,
};
pub const JinzaSafflower: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.5098039215686274f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const JitteryJade: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.9333333333333333f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const JohnLemon: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 1.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const JokerSSmile: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.00392156862745098f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const JollyJade: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.8f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const JovialJade: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const JubilantJade: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.6666666666666666f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const JubilantMeadow: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.7254901960784313f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const Juggernaut: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.3254901960784314f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const JuicyLime: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.8117647058823529f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const JuicyPeach: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.5725490196078431f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const Jumbo: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.5294117647058824f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const JuneBud: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.8549019607843137f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const Jungle: Colour = Colour {
    r: 0.0f32,
    g: 0.6431372549019608f32,
    b: 0.4f32,
    a: 1f32,
};
pub const JungleCivilization: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.403921568627451f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const JungleJam: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.3333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const JungleJewels: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.6509803921568628f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const JungleKing: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.30196078431372547f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const Jupiter: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8823529411764706f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const JurassicPark: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.4f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const JustPinkEnough: Colour = Colour {
    r: 1.0f32,
    g: 0.9215686274509803f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Kabuki: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.22745098039215686f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const Kabul: Colour = Colour {
    r: 0.4235294117647059f32,
    g: 0.3686274509803922f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const KaiserCheese: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8313725490196079f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const Kale: Colour = Colour {
    r: 0.39215686274509803f32,
    g: 0.5098039215686274f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const KaltesKlaresWasser: Colour = Colour {
    r: 0.058823529411764705f32,
    g: 0.996078431372549f32,
    b: 0.9764705882352941f32,
    a: 1f32,
};
pub const Kashmir: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.5529411764705883f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const Kathmandu: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.6039215686274509f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const KatyBerry: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Kefir: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.8352941176470589f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const Kelp: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.3137254901960784f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const KelpForest: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.5333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const KendallRose: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const KenyanCopper: Colour = Colour {
    r: 0.4235294117647059f32,
    g: 0.19607843137254902f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const KermitGreen: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.6980392156862745f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Ketchup: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.2196078431372549f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const KetchupLater: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.10980392156862745f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const Khaki: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.6901960784313725f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const KidIcarus: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.06274509803921569f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Kilimanjaro: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.20784313725490197f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const Kimchi: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.29411764705882354f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Kimono: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.5254901960784314f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const Kindleflame: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.5882352941176471f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const KingKong: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.0784313725490196f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const KingLime: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.8509803921568627f32,
    b: 0.0f32,
    a: 1f32,
};
pub const KingLizard: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.8666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const KingNacho: Colour = Colour {
    r: 1.0f32,
    g: 0.7215686274509804f32,
    b: 0.0f32,
    a: 1f32,
};
pub const KingNeptune: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.5803921568627451f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const KingOfWaves: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.8627450980392157f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const KingTriton: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.5215686274509804f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const KingSPlumPie: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.06274509803921569f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const KingfisherDaisy: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.20784313725490197f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const KinglyCloud: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8705882352941177f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const KingpinGold: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.6f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const KingsYellow: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8392156862745098f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const KinkyPinky: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Kirby: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.2823529411764706f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const Kirsch: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.07450980392156863f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const Kiss: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.5490196078431373f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const KissAFrog: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.7568627450980392f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const KissMeMore: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.4196078431372549f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const KissOfAVampire: Colour = Colour {
    r: 0.5411764705882353f32,
    g: 0.0f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const KissOfTheScorpion: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.2f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const Kissable: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.5607843137254902f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const KissedByMist: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const Kisses: Colour = Colour {
    r: 1.0f32,
    g: 0.4f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const KissesAndHugs: Colour = Colour {
    r: 1.0f32,
    g: 0.4f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const KittenSEye: Colour = Colour {
    r: 0.5411764705882353f32,
    g: 0.6784313725490196f32,
    b: 0.9686274509803922f32,
    a: 1f32,
};
pub const KittenSPaw: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.6588235294117647f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const KittyKitty: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.7411764705882353f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Kiwi: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.6196078431372549f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const KiwiCrush: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.7529411764705882f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const KiwiKiss: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9764705882352941f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const KnightRider: Colour = Colour {
    r: 0.058823529411764705f32,
    g: 0.027450980392156862f32,
    b: 0.027450980392156862f32,
    a: 1f32,
};
pub const KnightSArmor: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.36470588235294116f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const KnitCardigan: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.4235294117647059f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const KnockOnWood: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.6078431372549019f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const Kobe: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.17647058823529413f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const KöfteBrown: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.21176470588235294f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Koi: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.4f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const Kombucha: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.6235294117647059f32,
    b: 0.4f32,
    a: 1f32,
};
pub const KomodoDragon: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.5019607843137255f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const KoopaGreenShell: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.8470588235294118f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Koromiko: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.7098039215686275f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const KosherKhaki: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const KryptoniteGreen: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.6f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const LaLaLove: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.5647058823529412f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const LaLuna: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const LaPalma: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.5372549019607843f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const LaVibes: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const LaVieEnRose: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.6470588235294118f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const Labrador: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9254901960784314f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const LacqueredLiquorice: Colour = Colour {
    r: 0.2196078431372549f32,
    g: 0.2196078431372549f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const Lacrosse: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.3607843137254902f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Lagoon: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.6078431372549019f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const Laguna: Colour = Colour {
    r: 0.21176470588235294f32,
    g: 0.6470588235294118f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const LakeLucerne: Colour = Colour {
    r: 0.40784313725490196f32,
    g: 0.615686274509804f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const Laksa: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.7490196078431373f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const Lama: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.7333333333333333f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const LambsWool: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8196078431372549f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const Landjäger: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.25098039215686274f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const Langoustine: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.3215686274509804f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const LapisOnNeptune: Colour = Colour {
    r: 0.12156862745098039f32,
    g: 0.13333333333333333f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const LarbGai: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.7764705882352941f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const LaserLemon: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const LaserTrap: Colour = Colour {
    r: 1.0f32,
    g: 0.24705882352941178f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const LastOfLettuce: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.8666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const LastOfTheLilacs: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.7333333333333333f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const LastStraw: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.8588235294117647f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const LastWarning: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.058823529411764705f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const LastingLime: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const LaterGator: Colour = Colour {
    r: 0.0f32,
    g: 0.5411764705882353f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Latte: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.6470588235294118f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Lava: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.06274509803921569f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const LavaPit: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.43529411764705883f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const LavaRock: Colour = Colour {
    r: 0.3254901960784314f32,
    g: 0.3686274509803922f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const LavaStone: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.2549019607843137f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Lavender: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.43137254901960786f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const LavenderAsh: Colour = Colour {
    r: 0.6f32,
    g: 0.596078431372549f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const LavenderBliss: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.7647058823529411f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const LavenderCandy: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.7058823529411765f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const LavishSpending: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.4117647058823529f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const LawnGreen: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.6431372549019608f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const LazyDaisy: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9215686274509803f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const LazyLizard: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.611764705882353f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Lead: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.12941176470588237f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const Leaf: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.6666666666666666f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Leafy: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.6078431372549019f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const LeafyCanopy: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.8f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const LeafyLemon: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.9411764705882353f32,
    b: 0.0f32,
    a: 1f32,
};
pub const LeafyLush: Colour = Colour {
    r: 0.03137254901960784f32,
    g: 0.4117647058823529f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const LeafyWoodland: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.7333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Leapfrog: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.6627450980392157f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const Leather: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.41568627450980394f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Leek: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.8509803921568627f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const LeftOnRed: Colour = Colour {
    r: 1.0f32,
    g: 0.011764705882352941f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const LegendaryLavender: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.3803921568627451f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const Lemon: Colour = Colour {
    r: 1.0f32,
    g: 0.9686274509803922f32,
    b: 0.0f32,
    a: 1f32,
};
pub const LemonBurst: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8392156862745098f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const LemonCurd: Colour = Colour {
    r: 1.0f32,
    g: 0.9333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const LemonGrass: Colour = Colour {
    r: 0.6f32,
    g: 0.6039215686274509f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const LemonMeringue: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8823529411764706f32,
    b: 0.6f32,
    a: 1f32,
};
pub const LemonTart: Colour = Colour {
    r: 1.0f32,
    g: 0.8666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const LemonZest: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.8470588235294118f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const Lemonade: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8941176470588236f32,
    b: 0.6f32,
    a: 1f32,
};
pub const LesDemoisellesDAvignon: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.615686274509804f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const LetItSnow: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.9450980392156862f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const LethalLime: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 1.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Liaison: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.24705882352941178f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const Lichen: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.7294117647058823f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const LickAndKiss: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Lifeguard: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Lifeline: Colour = Colour {
    r: 0.6f32,
    g: 0.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const LightBlue: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.8470588235294118f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const LightBlush: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.7686274509803922f32,
    b: 0.8f32,
    a: 1f32,
};
pub const LightBrown: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.396078431372549f32,
    b: 0.11372549019607843f32,
    a: 1f32,
};
pub const LightGreen: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 1.0f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const LightGrey: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.8627450980392157f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const LightLilac: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.7764705882352941f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const LightMint: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 1.0f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const LightMyFire: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.3803921568627451f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const LightPink: Colour = Colour {
    r: 1.0f32,
    g: 0.8196078431372549f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const LightRed: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8274509803921568f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const LightSpirited: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.9333333333333333f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const LightYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.996078431372549f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Lighthouse: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9568627450980393f32,
    b: 0.9568627450980393f32,
    a: 1f32,
};
pub const LighthouseGlow: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8352941176470589f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const LightningBolt: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.9215686274509803f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const LightningBug: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8705882352941177f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const LightsOut: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.2784313725490196f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Lilac: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.6352941176470588f32,
    b: 0.9921568627450981f32,
    a: 1f32,
};
pub const LilacLace: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.6313725490196078f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const LilacLotion: Colour = Colour {
    r: 1.0f32,
    g: 0.2f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const LilacSpring: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Lily: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.6235294117647059f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const LilyPads: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.6901960784313725f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const Lima: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.9764705882352941f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const Lime: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 1.0f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const LimeFizz: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.9098039215686274f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const LimeMist: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 1.0f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const LimePunch: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.8431372549019608f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const LimeTwist: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.8392156862745098f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const LimeZest: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Limolicious: Colour = Colour {
    r: 0.592156862745098f32,
    g: 0.7176470588235294f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const Limon: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9215686274509803f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const Limonana: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.8666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Limoncello: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const LincolnGreen: Colour = Colour {
    r: 0.09803921568627451f32,
    g: 0.34901960784313724f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const LindwormGreen: Colour = Colour {
    r: 0.09019607843137255f32,
    g: 0.1568627450980392f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const Linen: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9411764705882353f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const LingeringStorm: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.5137254901960784f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const LinkToThePast: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.7058823529411765f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const Lion: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.6039215686274509f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const LionKing: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6f32,
    b: 0.2f32,
    a: 1f32,
};
pub const LionSRoar: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.8549019607843137f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Lionheart: Colour = Colour {
    r: 0.8f32,
    g: 0.13333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const LipGloss: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.803921568627451f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Lipstick: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.3568627450980392f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const LipstickIllusion: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.4117647058823529f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const LiquidGold: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.7764705882352941f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const LiquidLava: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.4588235294117647f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const LiquidLime: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.9725490196078431f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const LiquidNeon: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Liquorice: Colour = Colour {
    r: 0.0392156862745098f32,
    g: 0.0196078431372549f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const LiquoriceRed: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.03529411764705882f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Lit: Colour = Colour {
    r: 1.0f32,
    g: 0.996078431372549f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const LittleLadybug: Colour = Colour {
    r: 1.0f32,
    g: 0.0784313725490196f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const LittleLamb: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9019607843137255f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const LittleMermaid: Colour = Colour {
    r: 0.17647058823529413f32,
    g: 0.27058823529411763f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const LittlePrincess: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.6666666666666666f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const Liver: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.2901960784313726f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const Lizard: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.4117647058823529f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const Llilacquered: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.3568627450980392f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Lobster: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.1411764705882353f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const LocalCurry: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.6196078431372549f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const LochNess: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.42745098039215684f32,
    b: 0.6901960784313725f32,
    a: 1f32,
};
pub const Lolita: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.15294117647058825f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const Lollipop: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.11764705882352941f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const LoneHunter: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.7843137254901961f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const LonelyChocolate: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.0392156862745098f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Lonestar: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.1411764705882353f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const LongBeach: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9372549019607843f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const LonghaulFlight: Colour = Colour {
    r: 0.0f32,
    g: 0.13333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const LooneyBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 1.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const LordsOfTheNight: Colour = Colour {
    r: 0.4f32,
    g: 0.26666666666666666f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const LostAtSea: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.611764705882353f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const LostGolfer: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.6862745098039216f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const LostInHeaven: Colour = Colour {
    r: 0.0f32,
    g: 0.1411764705882353f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const LostInSpace: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.2196078431372549f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const LostInTheWoods: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.26666666666666666f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const LostInTime: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.6862745098039216f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const LostSpace: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.5764705882352941f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const Lotion: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9921568627450981f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const LotusFlower: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.9411764705882353f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const LoudLime: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 1.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const LoudiciousPink: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.1843137254901961f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const LoveAffair: Colour = Colour {
    r: 1.0f32,
    g: 0.7450980392156863f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const LoveDust: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.5803921568627451f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const LoveFumes: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8156862745098039f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const LoveGoddess: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.050980392156862744f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const LoveJuice: Colour = Colour {
    r: 0.8f32,
    g: 0.06666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const LoveLetter: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.396078431372549f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const LovePotion: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.0784313725490196f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const LovePriestess: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const LoveSpell: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.7058823529411765f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const LoveSurge: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.11372549019607843f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const LoveVessel: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Loveland: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.44313725490196076f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const LovelyBreeze: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.8470588235294118f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const LovelyLittleRosy: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.37254901960784315f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Lox: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.5647058823529412f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const LucidDream: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.1843137254901961f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const LucidDreams: Colour = Colour {
    r: 0.8f32,
    g: 0.9333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const LuciusLilac: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.6352941176470588f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const Lucky: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.6039215686274509f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const LuckyClover: Colour = Colour {
    r: 0.0f32,
    g: 0.5176470588235295f32,
    b: 0.0f32,
    a: 1f32,
};
pub const LuckyGrey: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.4666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const LuckyLobster: Colour = Colour {
    r: 0.8f32,
    g: 0.2f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const LuckyPenny: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.43529411764705883f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const LudicrousLemming: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Luigi: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.7333333333333333f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const Lumberjack: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.27058823529411763f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const Luna: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.8470588235294118f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const LunarBase: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.5294117647058824f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const LunarLanding: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.8117647058823529f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const LunarLight: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.5843137254901961f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const LunarLuxury: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9568627450980393f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const LunarOutpost: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.5098039215686274f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const LunaticLynx: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const LuridLettuce: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.9529411764705882f32,
    b: 0.09803921568627451f32,
    a: 1f32,
};
pub const LusciousLemongrass: Colour = Colour {
    r: 0.3176470588235294f32,
    g: 0.4745098039215686f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Lush: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.7411764705882353f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const LushBamboo: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const LushGarden: Colour = Colour {
    r: 0.0f32,
    g: 0.5333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const LushGrass: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.5529411764705883f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const LushGreen: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const LushParadise: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.49019607843137253f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const LushPlains: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Lust: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.12549019607843137f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const LustfulWishes: Colour = Colour {
    r: 0.8f32,
    g: 0.26666666666666666f32,
    b: 0.6f32,
    a: 1f32,
};
pub const LustyLavender: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.3686274509803922f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const LustyLips: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.09019607843137255f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const LustyLizard: Colour = Colour {
    r: 0.0f32,
    g: 0.7333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const LuxorGold: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.5529411764705883f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const Luxurious: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.7176470588235294f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const LuxuriousLime: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Lynx: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.30196078431372547f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const MBison: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.00784313725490196f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const Macabre: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Macadamia: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const Macaroni: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8156862745098039f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const MacaroniAndCheese: Colour = Colour {
    r: 1.0f32,
    g: 0.7254901960784313f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const Macaroon: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.5450980392156862f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const Macau: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.7607843137254902f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Machinery: Colour = Colour {
    r: 0.6f32,
    g: 0.6f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const MachuPicchuGardens: Colour = Colour {
    r: 0.6f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const MadForMango: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.6352941176470588f32,
    b: 0.0f32,
    a: 1f32,
};
pub const MadeInTheShade: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.44313725490196076f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const MademoisellePink: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.01568627450980392f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const Madonna: Colour = Colour {
    r: 0.24705882352941178f32,
    g: 0.25882352941176473f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const Madras: Colour = Colour {
    r: 0.2784313725490196f32,
    g: 0.24313725490196078f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const Magenta: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const MagentaAffair: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.26666666666666666f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const MagentaElephant: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.00392156862745098f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const MagentaFizz: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.1411764705882353f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const MagentaMemoir: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.3333333333333333f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Magentarama: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.20392156862745098f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const Magentle: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Magentleman: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.13333333333333333f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Magento: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.23529411764705882f32,
    b: 1.0f32,
    a: 1f32,
};
pub const MagicCarpet: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.5333333333333333f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const MagicInk: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.2784313725490196f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const MagicMagenta: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.2784313725490196f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const MagicPotion: Colour = Colour {
    r: 1.0f32,
    g: 0.26666666666666666f32,
    b: 0.4f32,
    a: 1f32,
};
pub const MagicalMerlin: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.5568627450980392f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const MagicalMoonlight: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9333333333333333f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const MagicalStardust: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9176470588235294f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const Magma: Colour = Colour {
    r: 1.0f32,
    g: 0.3058823529411765f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const MagnaCumLaude: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Magnesium: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.7607843137254902f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const Magnet: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.3137254901960784f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Magnetic: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.7098039215686275f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const MagnificentMagenta: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Magnolia: Colour = Colour {
    r: 1.0f32,
    g: 0.9764705882352941f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const MagnoliaPetal: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9333333333333333f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const Maharaja: Colour = Colour {
    r: 0.24705882352941178f32,
    g: 0.20784313725490197f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const Mahogany: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.25098039215686274f32,
    b: 0.0f32,
    a: 1f32,
};
pub const MaiTai: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.396078431372549f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const MaidenSBlush: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8274509803921568f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const MaisonVerte: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.9411764705882353f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const Maize: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8156862745098039f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Maizena: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9254901960784314f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const MajesticDune: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.7372549019607844f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const MajesticEggplant: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.2f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const MajesticEvergreen: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.5333333333333333f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const MajesticMagenta: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.26666666666666666f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const MajesticMagic: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const Majesty: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.24313725490196078f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const MajorBrown: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.3411764705882353f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const MajorMagenta: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.27450980392156865f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const MajorelleGardens: Colour = Colour {
    r: 0.2f32,
    g: 0.4666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const MakinItRain: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const MalevolentMauve: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.4f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Malibu: Colour = Colour {
    r: 0.4f32,
    g: 0.7176470588235294f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const MalibuSun: Colour = Colour {
    r: 1.0f32,
    g: 0.9490196078431372f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const Mallard: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.2823529411764706f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Malt: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8117647058823529f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const Mamba: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.42745098039215684f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const ManCave: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.3764705882352941f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const ManaTree: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.4745098039215686f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const Mandarin: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.47843137254901963f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const MandarinRind: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.5647058823529412f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const MangaPink: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.7254901960784313f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const MangalaPink: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.5058823529411764f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const Mango: Colour = Colour {
    r: 1.0f32,
    g: 0.6509803921568628f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const MangoCheesecake: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9294117647058824f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const MangoLatte: Colour = Colour {
    r: 1.0f32,
    g: 0.7333333333333333f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const MangoMadness: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.5490196078431373f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const MangoTango: Colour = Colour {
    r: 1.0f32,
    g: 0.5098039215686274f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const Mangrove: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.4549019607843137f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const Manhattan: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.6862745098039216f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const ManiacGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.5647058823529412f32,
    b: 0.0f32,
    a: 1f32,
};
pub const ManiacMansion: Colour = Colour {
    r: 0.0f32,
    g: 0.25098039215686274f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Mantis: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.7647058823529411f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const MapleSyrup: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.5764705882352941f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Maraschino: Colour = Colour {
    r: 1.0f32,
    g: 0.14901960784313725f32,
    b: 0.0f32,
    a: 1f32,
};
pub const MarbleGrape: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8862745098039215f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const MarbleQuarry: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.8627450980392157f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const MarbleWhite: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9411764705882353f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Marigold: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.7529411764705882f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const MarilynMonrouge: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.0f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const Marina: Colour = Colour {
    r: 0.35294117647058826f32,
    g: 0.5333333333333333f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const MarinaraRed: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const Marine: Colour = Colour {
    r: 0.01568627450980392f32,
    g: 0.1803921568627451f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const Mario: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.0f32,
    b: 0.058823529411764705f32,
    a: 1f32,
};
pub const Maritime: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.8117647058823529f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const MaritimeOutpost: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.27058823529411763f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const Maroon: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Mars: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.3843137254901961f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const MarshFog: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.8470588235294118f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Marshmallow: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9333333333333333f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const MarshmallowHeart: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.8627450980392157f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const Marsupilami: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9490196078431372f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Martian: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.6313725490196078f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const MartianCerulean: Colour = Colour {
    r: 0.3411764705882353f32,
    g: 0.5843137254901961f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const MartianColony: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.4588235294117647f32,
    b: 0.058823529411764705f32,
    a: 1f32,
};
pub const Martini: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.6588235294117647f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const Masala: Colour = Colour {
    r: 0.3411764705882353f32,
    g: 0.3254901960784314f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Mascarpone: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.9019607843137255f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const MasterChief: Colour = Colour {
    r: 0.3137254901960784f32,
    g: 0.49019607843137253f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const MasterKey: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const MasterNacho: Colour = Colour {
    r: 1.0f32,
    g: 0.7215686274509804f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const MasterSwordBlue: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const MatchaMecha: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.6862745098039216f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const MattBlack: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.08235294117647059f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const MattBlue: Colour = Colour {
    r: 0.17254901960784313f32,
    g: 0.43529411764705883f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const MattDemon: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.26666666666666666f32,
    b: 0.2f32,
    a: 1f32,
};
pub const MattGreen: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.6784313725490196f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const MattLilac: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.7764705882352941f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const MattPink: Colour = Colour {
    r: 1.0f32,
    g: 0.7137254901960784f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const MattPurple: Colour = Colour {
    r: 0.5764705882352941f32,
    g: 0.4392156862745098f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const MattWhite: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const Matterhorn: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.29411764705882354f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const MatureCognac: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.27450980392156865f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const Mauve: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.6901960784313725f32,
    b: 1.0f32,
    a: 1f32,
};
pub const MauveIt: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.26666666666666666f32,
    b: 0.4f32,
    a: 1f32,
};
pub const MauveMagic: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.5686274509803921f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const Mauvelous: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.7019607843137254f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const MayGreen: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.5686274509803921f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const MayanTreasure: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.596078431372549f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Mayonnaise: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9333333333333333f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const Mcnuke: Colour = Colour {
    r: 0.2f32,
    g: 1.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const MeadowMorn: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.7450980392156863f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const MeadowYellow: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8549019607843137f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const MeanGirlsLipstick: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.6823529411764706f32,
    a: 1f32,
};
pub const Meat: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.5019607843137255f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const Meatloaf: Colour = Colour {
    r: 0.4f32,
    g: 0.2f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const MechaKitty: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.7686274509803922f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const MechaMetal: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.5137254901960784f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const Medallion: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.6509803921568628f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const MedievalBlue: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.2196078431372549f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Mediterranea: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.38823529411764707f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const MediterraneanBlue: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.5098039215686274f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const MediterraneanSea: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.5490196078431373f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const MediumRoast: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.12549019607843137f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const Medlar: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.8431372549019608f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const MegaMetalMecha: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.796078431372549f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const Megaman: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.7372549019607844f32,
    b: 0.9882352941176471f32,
    a: 1f32,
};
pub const Melancholia: Colour = Colour {
    r: 0.07058823529411765f32,
    g: 0.2235294117647059f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const Melanzane: Colour = Colour {
    r: 0.20392156862745098f32,
    g: 0.1607843137254902f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const MellowApricot: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.7215686274509804f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const MellowMango: Colour = Colour {
    r: 0.8f32,
    g: 0.26666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const MellowMelon: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const MellowMint: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.9294117647058824f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const MellowYellow: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8705882352941177f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const MelodramaticMagenta: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Melon: Colour = Colour {
    r: 1.0f32,
    g: 0.47058823529411764f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Melondrama: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.5058823529411764f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const MeltingGlacier: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.9764705882352941f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const MeltingPoint: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.8823529411764706f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const MemoryLane: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.8196078431372549f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const MentalFloss: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.7058823529411765f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const MentholKiss: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.8862745098039215f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const Mercurial: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.6901960784313725f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const Mercury: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.9215686274509803f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const Merguez: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.0f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const Meringue: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8941176470588236f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Merino: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8588235294117647f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const Merlot: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.0f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const MerlotFields: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.15294117647058825f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const MerlotMagic: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.25098039215686274f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const MermaidBlues: Colour = Colour {
    r: 0.0f32,
    g: 0.26666666666666666f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const MermaidDreams: Colour = Colour {
    r: 0.0f32,
    g: 0.5333333333333333f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const MermaidTears: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.9019607843137255f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const MermaidSKiss: Colour = Colour {
    r: 0.34901960784313724f32,
    g: 0.7843137254901961f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const Metal: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.7490196078431373f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const MetalGear: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.7647058823529411f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const MetalPetal: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.5647058823529412f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const Metallic: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.7647058823529411f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Meteor: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.4549019607843137f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const MeteorShower: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.2f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Meteorite: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.23137254901960785f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const Methadone: Colour = Colour {
    r: 0.8f32,
    g: 0.13333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const MetroidRed: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.2196078431372549f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Metropolis: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.34509803921568627f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const MetropolitanSilhouette: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.25882352941176473f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const MexicanChile: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.42745098039215684f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const MexicanStandoff: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.6235294117647059f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const Microchip: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.7372549019607844f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const Midnight: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.00392156862745098f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const MidnightDreams: Colour = Colour {
    r: 0.0f32,
    g: 0.13333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const MidnightExpress: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.14901960784313725f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const MidnightInTokyo: Colour = Colour {
    r: 0.0f32,
    g: 0.0f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const MidnightInterlude: Colour = Colour {
    r: 0.19607843137254902f32,
    g: 0.28627450980392155f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const MidnightMelancholia: Colour = Colour {
    r: 0.0f32,
    g: 0.13333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const MidnightPie: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.17647058823529413f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const MidnightPines: Colour = Colour {
    r: 0.09019607843137255f32,
    g: 0.1411764705882353f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const MidnightSerenade: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.2627450980392157f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const MidnightShadow: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.38823529411764707f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const MidnightSky: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.2784313725490196f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const Midori: Colour = Colour {
    r: 0.16470588235294117f32,
    g: 0.3764705882352941f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const MidsummerNights: Colour = Colour {
    r: 0.0f32,
    g: 0.06666666666666667f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const MightyMauve: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.4980392156862745f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const MikadoYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.7686274509803922f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const MilitantVegan: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.6f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Milk: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 1.0f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const MilkAndCookies: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.8823529411764706f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const MilkChocolate: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.3058823529411765f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const MilkFoam: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 1.0f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const MilkMustache: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9529411764705882f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const MilkTooth: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9215686274509803f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const Millefeuille: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.7843137254901961f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const MillennialPink: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.7843137254901961f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const MillionGrey: Colour = Colour {
    r: 0.6f32,
    g: 0.6f32,
    b: 0.6f32,
    a: 1f32,
};
pub const MimiPink: Colour = Colour {
    r: 1.0f32,
    g: 0.8549019607843137f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const Mimosa: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9137254901960784f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const Mindaro: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.9176470588235294f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const Minestrone: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.14901960784313725f32,
    b: 0.08627450980392157f32,
    a: 1f32,
};
pub const Ming: Colour = Colour {
    r: 0.25098039215686274f32,
    g: 0.4588235294117647f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const MinionYellow: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.807843137254902f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Mink: Colour = Colour {
    r: 0.5411764705882353f32,
    g: 0.4588235294117647f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const MinotaurusBrown: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Mint: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.7058823529411765f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const MintBliss: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 1.0f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const MintChip: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.9215686274509803f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const MintCoffee: Colour = Colour {
    r: 0.8f32,
    g: 1.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const MintTwist: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.796078431372549f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const Mintolicious: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.9137254901960784f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const Mintastic: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 1.0f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const MintedBlueberryLemonade: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.14901960784313725f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Mintnight: Colour = Colour {
    r: 0.48627450980392156f32,
    g: 0.7333333333333333f32,
    b: 0.6823529411764706f32,
    a: 1f32,
};
pub const MintyFresh: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.9490196078431372f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const MintyParadise: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const MinuteMauve: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8941176470588236f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const MississippiRiver: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.38823529411764707f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const MistyColdSea: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.7333333333333333f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const MistyHarbor: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.4627450980392157f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const MistyHaze: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.788235294117647f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const MistyMarsh: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.8823529411764706f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const MistyMorning: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.7843137254901961f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const MistyMountains: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.8156862745098039f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Mithril: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.5294117647058824f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const MiyamotoRed: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.011764705882352941f32,
    b: 0.058823529411764705f32,
    a: 1f32,
};
pub const Moccasin: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9215686274509803f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const Mocha: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.4627450980392157f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const MochaIce: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8235294117647058f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const MochaMadness: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.4196078431372549f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Mochaccino: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.3215686274509804f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Mochito: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.9803921568627451f32,
    b: 0.0f32,
    a: 1f32,
};
pub const ModernMonument: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.8392156862745098f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const MoelleuxAuChocolat: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.2f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Mohalla: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.6078431372549019f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const Mojito: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.9529411764705882f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const Molasses: Colour = Colour {
    r: 0.3411764705882353f32,
    g: 0.2901960784313726f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const Mole: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.17647058823529413f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const MoltenCaramel: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.47843137254901963f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const MoltenCore: Colour = Colour {
    r: 1.0f32,
    g: 0.34509803921568627f32,
    b: 0.0f32,
    a: 1f32,
};
pub const MoltenGold: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.7764705882352941f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const MomSPancake: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.7725490196078432f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const MomoPeach: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.4745098039215686f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const MonaLisa: Colour = Colour {
    r: 1.0f32,
    g: 0.596078431372549f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const MonetMagic: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.6745098039215687f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const Money: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.6039215686274509f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const MonkeyIsland: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.23137254901960785f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const Monstera: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.403921568627451f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const MonstrousGreen: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.8f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const MontBlanc: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.7137254901960784f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const MontezumaGold: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const MontreuxBlue: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.4745098039215686f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const Monument: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.5372549019607843f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const MonumentValley: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.3607843137254902f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Monza: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.011764705882352941f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const MoonBase: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.49019607843137253f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const MoonGlow: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9529411764705882f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const MoonLanding: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.6549019607843137f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const MoonRock: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.49019607843137253f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const MoonVeil: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.6f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const Moonbeam: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.7215686274509804f32,
    b: 0.6823529411764706f32,
    a: 1f32,
};
pub const Moondance: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8705882352941177f32,
    b: 0.8f32,
    a: 1f32,
};
pub const MoonlessMystery: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.1411764705882353f32,
    b: 0.2f32,
    a: 1f32,
};
pub const MoonlessNight: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.2235294117647059f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const MoonlessSky: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.29411764705882354f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Moonlight: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9333333333333333f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const MoonlightMauve: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.5137254901960784f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const MoonlitForest: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.42745098039215684f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const Moonraker: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.6980392156862745f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const Moonscape: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.4196078431372549f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Moonwalk: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.7450980392156863f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const Moormonster: Colour = Colour {
    r: 0.12156862745098039f32,
    g: 0.32941176470588235f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const Moorland: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.6705882352941176f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const MorbidPrincess: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.054901960784313725f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const Morel: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.39215686274509803f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const MorningBread: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.9019607843137255f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const MorningMist: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.9294117647058824f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const MorningSnow: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9568627450980393f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const MoroccanBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.33725490196078434f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const Morocco: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.4470588235294118f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const MorrisLeaf: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.8274509803921568f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const Mosque: Colour = Colour {
    r: 0.0f32,
    g: 0.37254901960784315f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const Moss: Colour = Colour {
    r: 0.0f32,
    g: 0.5647058823529412f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Mosslands: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.6f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Mossy: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.45098039215686275f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const MossyGlossy: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.6078431372549019f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Moth: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.7568627450980392f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const MotherEarth: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.611764705882353f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const MotherNature: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.8823529411764706f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const MotherSMilk: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9294117647058824f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const Motherland: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.7137254901960784f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const Mothy: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.7333333333333333f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const MountEden: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.9372549019607843f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const MountOlympus: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 1.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const MountainDew: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.8862745098039215f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const MountainPeak: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.8784313725490196f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const MountainView: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.2980392156862745f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const MoutardeDeBénichon: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.5647058823529412f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const MsPacmanKiss: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const MtRushmore: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.5058823529411764f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const Muddy: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.2235294117647059f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const MuddyBrown: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.40784313725490196f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const MuddyGreen: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.4549019607843137f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const MuddyMauve: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.7019607843137254f32,
    b: 0.8f32,
    a: 1f32,
};
pub const MuddyOlive: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.36470588235294116f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const MuddyQuicksand: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.596078431372549f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const MuddyRose: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.7450980392156863f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const MuddyYellow: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.6745098039215687f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const Muesli: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.49411764705882355f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const MuffledWhite: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8588235294117647f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const Mulberry: Colour = Colour {
    r: 0.5725490196078431f32,
    g: 0.0392156862745098f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Mule: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.4823529411764706f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const MulledCider: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.5058823529411764f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const MummySTomb: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.5568627450980392f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const MunchOnMelon: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.24313725490196078f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const MunsellBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.5764705882352941f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const MunsellYellow: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Murasaki: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.1568627450980392f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const MurderousMagenta: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.12549019607843137f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const Murmur: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.8117647058823529f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const MuscatBlanc: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8862745098039215f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const Mushroom: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.6745098039215687f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const MushroomForest: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.5019607843137255f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const MushroomRisotto: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.8156862745098039f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const Mustang: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.2901960784313726f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const Mustard: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.7019607843137254f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const MustardMusketeers: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.6313725490196078f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const MustardSeed: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.6235294117647059f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const Mutabilis: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.5843137254901961f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const MutedBerry: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.47058823529411764f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const MutedBlue: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.44313725490196076f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const MutedClay: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.5411764705882353f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const MutedGreen: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.6274509803921569f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const MutedLime: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.7764705882352941f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const MutedMauve: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.6627450980392157f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const MutedPink: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.4627450980392157f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const MutedPurple: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.3568627450980392f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const MvsRed: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Mykonos: Colour = Colour {
    r: 0.2196078431372549f32,
    g: 0.47843137254901963f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const Myrtle: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.25882352941176473f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const MysteriousBlue: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.47843137254901963f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const MysteriousDepths: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.03529411764705882f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const MysteriousMixture: Colour = Colour {
    r: 0.058823529411764705f32,
    g: 0.3215686274509804f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const MysteriousWaters: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.27058823529411763f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const MysteryMint: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.9372549019607843f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const MysticBlue: Colour = Colour {
    r: 0.2823529411764706f32,
    g: 0.6588235294117647f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const MysticMagenta: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.1803921568627451f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const MysticWhite: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.9215686274509803f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const MysticalShadow: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.16862745098039217f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const MystifyingMagenta: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.12549019607843137f32,
    b: 0.6901960784313725f32,
    a: 1f32,
};
pub const Nacho: Colour = Colour {
    r: 1.0f32,
    g: 0.796078431372549f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const NachoCheese: Colour = Colour {
    r: 1.0f32,
    g: 0.7333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const NagaViperPepper: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.1607843137254902f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const NakedNoodle: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.796078431372549f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const Namibia: Colour = Colour {
    r: 0.48627450980392156f32,
    g: 0.42745098039215684f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const NaplesYellow: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.8549019607843137f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const NapoleonicBlue: Colour = Colour {
    r: 0.17254901960784313f32,
    g: 0.2549019607843137f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const NarwhalGrey: Colour = Colour {
    r: 0.03137254901960784f32,
    g: 0.03137254901960784f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const Nattō: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.596078431372549f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const NaturalLight: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9215686274509803f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const NaturalOrchestra: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.611764705882353f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const NaturalOrder: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.6901960784313725f32,
    b: 0.2f32,
    a: 1f32,
};
pub const NaturalWool: Colour = Colour {
    r: 1.0f32,
    g: 0.9647058823529412f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const Nature: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.8352941176470589f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const NaughtyHottie: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.25098039215686274f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const Nautical: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.2901960784313726f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const NauticalCreatures: Colour = Colour {
    r: 0.1607843137254902f32,
    g: 0.3607843137254902f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Nautilus: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.23529411764705882f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const Naval: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.4470588235294118f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const NavalAdventures: Colour = Colour {
    r: 0.027450980392156862f32,
    g: 0.14901960784313725f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const NavalBlue: Colour = Colour {
    r: 0.2196078431372549f32,
    g: 0.29411764705882354f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const NavalNight: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.10980392156862745f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const NearMoon: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.9058823529411765f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Nebula: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.01568627450980392f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const Nebulous: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.7254901960784313f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const NecklacePearl: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9333333333333333f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Nectar: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.8549019607843137f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const NectarJackpot: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8274509803921568f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const Nectarine: Colour = Colour {
    r: 1.0f32,
    g: 0.5254901960784314f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const NegishiGreen: Colour = Colour {
    r: 0.5764705882352941f32,
    g: 0.5450980392156862f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const Negroni: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7803921568627451f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const NeoMint: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 1.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const NeonBlue: Colour = Colour {
    r: 0.01568627450980392f32,
    g: 0.8509803921568627f32,
    b: 1.0f32,
    a: 1f32,
};
pub const NeonBoneyard: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.7725490196078432f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const NeonCarrot: Colour = Colour {
    r: 1.0f32,
    g: 0.596078431372549f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const NeonFuchsia: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.2549019607843137f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const NeonGreen: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 1.0f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const NeonPink: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.00392156862745098f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const NeonPurple: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.07450980392156863f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const NeonRed: Colour = Colour {
    r: 1.0f32,
    g: 0.027450980392156862f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const NeonRomance: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.00784313725490196f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const NeonRose: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const NeonViolet: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.2823529411764706f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const NeonYellow: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 1.0f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const NeptuneGreen: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.7333333333333333f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const Nero: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.1450980392156863f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const NervousNeonPink: Colour = Colour {
    r: 1.0f32,
    g: 0.43137254901960786f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Netherworld: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Nettle: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.6745098039215687f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const NeverForget: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.4470588235294118f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const NevermindNirvana: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.7843137254901961f32,
    b: 0.9647058823529412f32,
    a: 1f32,
};
pub const NewGold: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8196078431372549f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const NewHeights: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.8980392156862745f32,
    b: 0.9490196078431372f32,
    a: 1f32,
};
pub const NewLove: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.7333333333333333f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const NiagaraFalls: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.8901960784313725f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Nickel: Colour = Colour {
    r: 0.5725490196078431f32,
    g: 0.5725490196078431f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const NicotineGold: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const NightEdition: Colour = Colour {
    r: 0.12549019607843137f32,
    g: 0.34509803921568627f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const NightKite: Colour = Colour {
    r: 0.0f32,
    g: 0.3333333333333333f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const NightMarket: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.3803921568627451f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const NightMode: Colour = Colour {
    r: 0.13725490196078433f32,
    g: 0.3058823529411765f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const NightOwl: Colour = Colour {
    r: 0.36470588235294116f32,
    g: 0.4823529411764706f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const NightRendezvous: Colour = Colour {
    r: 0.4f32,
    g: 0.47058823529411764f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const NightRider: Colour = Colour {
    r: 0.2f32,
    g: 0.1803921568627451f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const NightSky: Colour = Colour {
    r: 0.1607843137254902f32,
    g: 0.16862745098039217f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const NightSnow: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.8f32,
    b: 1.0f32,
    a: 1f32,
};
pub const NightWatch: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.30980392156862746f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Nightfall: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.3254901960784314f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const Nightlife: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.25882352941176473f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const NightlyActivities: Colour = Colour {
    r: 0.1803921568627451f32,
    g: 0.3137254901960784f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const NightlyExpedition: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const NightlyWalk: Colour = Colour {
    r: 0.32941176470588235f32,
    g: 0.27058823529411763f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Nightmare: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Nile: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.7254901960784313f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const NimbusCloud: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.7843137254901961f32,
    b: 0.8f32,
    a: 1f32,
};
pub const NinjaPrincess: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.3215686274509804f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const NinjaTurtle: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.6941176470588235f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const Nipple: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.4666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const Nippon: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.0f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const Nirvana: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.5686274509803921f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const NoWayRosé: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.6666666666666666f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const NobleBlack: Colour = Colour {
    r: 0.12549019607843137f32,
    g: 0.12941176470588237f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const NobleChocolate: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.26666666666666666f32,
    b: 0.2f32,
    a: 1f32,
};
pub const NobleCream: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8549019607843137f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const NobleKnight: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.30196078431372547f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const NoblePlum: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.12156862745098039f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const NobleRed: Colour = Colour {
    r: 0.5725490196078431f32,
    g: 0.09411764705882353f32,
    b: 0.11372549019607843f32,
    a: 1f32,
};
pub const Nocturnal: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.49019607843137253f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const NocturnalExpedition: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.2980392156862745f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const Nocturne: Colour = Colour {
    r: 0.20392156862745098f32,
    g: 0.30196078431372547f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const NocturneBlue: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.3215686274509804f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const NocturneRed: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.29411764705882354f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const Noir: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.16862745098039217f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const NoirFiction: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.03137254901960784f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Nomad: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.6f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const Noodles: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.8901960784313725f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const NordicBreeze: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.8666666666666667f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const NordicNoir: Colour = Colour {
    r: 0.0f32,
    g: 0.2f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const NorthAtlantic: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.4823529411764706f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const NorthStar: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8705882352941177f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const NorthernStar: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const Northwind: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.8980392156862745f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const Norway: Colour = Colour {
    r: 0.6431372549019608f32,
    g: 0.7215686274509804f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const NotYetCaramel: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.44313725490196076f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const NotYoCheese: Colour = Colour {
    r: 1.0f32,
    g: 0.7568627450980392f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const Nougat: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.5411764705882353f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const Nouveauriche: Colour = Colour {
    r: 1.0f32,
    g: 0.7333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const NovelLilac: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.6431372549019608f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const Noxious: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.6352941176470588f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const NuclearAcid: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.9568627450980393f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const NuclearBlast: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const NuclearMango: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6f32,
    b: 0.2f32,
    a: 1f32,
};
pub const NuclearMeltdown: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.9333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const NuclearThrone: Colour = Colour {
    r: 0.0f32,
    g: 0.8705882352941177f32,
    b: 0.0f32,
    a: 1f32,
};
pub const NudeFlamingo: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.5607843137254902f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const NudeLips: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.5803921568627451f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const Nugget: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.5725490196078431f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const NuitBlanche: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.2823529411764706f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const NutCracker: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.4235294117647059f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const Nutmeg: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.2901960784313726f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const NycTaxi: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.7176470588235294f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Nylon: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.8901960784313725f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const NymphSDelight: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.4235294117647059f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const OTannenbaum: Colour = Colour {
    r: 0.0f32,
    g: 0.3333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const OakBarrel: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.33725490196078434f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const OakPalace: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.4196078431372549f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const Oakwood: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.6470588235294118f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const Oasis: Colour = Colour {
    r: 0.0f32,
    g: 0.5725490196078431f32,
    b: 0.6392156862745098f32,
    a: 1f32,
};
pub const OatMilk: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8549019607843137f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Oatmeal: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.7568627450980392f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const OatmealCookie: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8549019607843137f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const Oblivion: Colour = Colour {
    r: 0.0f32,
    g: 0.01568627450980392f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const ObscureOgre: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.09803921568627451f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const Obsidian: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.3137254901960784f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const ObsidianShard: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.011764705882352941f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const OceanBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.615686274509804f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const OceanBlues: Colour = Colour {
    r: 0.3137254901960784f32,
    g: 0.5254901960784314f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const OceanBreeze: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.8980392156862745f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const OceanSlumber: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.4627450980392157f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const OceanSEmbrace: Colour = Colour {
    r: 0.18823529411764706f32,
    g: 0.41568627450980394f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const OceanicMotion: Colour = Colour {
    r: 0.11372549019607843f32,
    g: 0.3607843137254902f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const OchreSpice: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.42745098039215684f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const OhBoy: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8549019607843137f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const OhEmGhee: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.7843137254901961f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const OhMyGold: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const OhPistachio: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.792156862745098f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Oil: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.2f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const OldGold: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.7098039215686275f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const OldHeart: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.41568627450980394f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const OldRose: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.5019607843137255f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const OldSilver: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.5176470588235295f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const OldWhiskey: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const OldWorld: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.6588235294117647f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const Olivary: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.34901960784313724f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const Olive: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.5019607843137255f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const OliveBark: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.3333333333333333f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const OliveConqueringWhite: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.8980392156862745f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const OliveLeaf: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.29411764705882354f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const OliveNiçoise: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.2627450980392157f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const OliveTree: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.6549019607843137f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const Olivia: Colour = Colour {
    r: 0.6f32,
    g: 0.4f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const OlympianBlue: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.2980392156862745f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const OlympicBlue: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.5607843137254902f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const OlympusWhite: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.8470588235294118f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const OnCloudNine: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.9058823529411765f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const OnceBitten: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.1843137254901961f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const OnceInABlueMoon: Colour = Colour {
    r: 0.0f32,
    g: 0.26666666666666666f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const OneMinuteToMidnight: Colour = Colour {
    r: 0.0f32,
    g: 0.2f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const OneYearOfRain: Colour = Colour {
    r: 0.1607843137254902f32,
    g: 0.27450980392156865f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const OnionSkin: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9294117647058824f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Onsen: Colour = Colour {
    r: 0.4f32,
    g: 0.9333333333333333f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Onyx: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.27058823529411763f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Opal: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.8784313725490196f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const OpalFire: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.611764705882353f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const OpalFlame: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.3607843137254902f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const OpalGreen: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.4745098039215686f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const OpenBook: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9450980392156862f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const OpenSeas: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.6862745098039216f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const Opera: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.396078431372549f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const Opium: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.49411764705882355f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const OpulentBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.3333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const OpulentLime: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const OpulentOrange: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.4f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const OpulentPurple: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.2f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const OpulentTurquoise: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Orange: Colour = Colour {
    r: 1.0f32,
    g: 0.6470588235294118f32,
    b: 0.0f32,
    a: 1f32,
};
pub const OrangeClownFish: Colour = Colour {
    r: 1.0f32,
    g: 0.3333333333333333f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const OrangeCrush: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const OrangeDelight: Colour = Colour {
    r: 1.0f32,
    g: 0.7647058823529411f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const OrangeJuice: Colour = Colour {
    r: 1.0f32,
    g: 0.4980392156862745f32,
    b: 0.0f32,
    a: 1f32,
};
pub const OrangePiñata: Colour = Colour {
    r: 1.0f32,
    g: 0.4f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Orangealicious: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Orangina: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.7764705882352941f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const OrbOfDiscord: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const OrbOfHarmony: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Orbital: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.5137254901960784f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const OrbitalKingdom: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.0f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Orchid: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.5058823529411764f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Organic: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.4470588235294118f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const OrientalBlush: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.7764705882352941f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const OrientalHerbs: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.5333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const OrientalNights: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.17254901960784313f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const OrientalOlive: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.3333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const OrientalPink: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.5568627450980392f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const OrientalRuby: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.3254901960784314f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const OrientalScent: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.7490196078431373f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const OrientalSilk: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8980392156862745f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const OrientalSpice: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.3176470588235294f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Oriole: Colour = Colour {
    r: 1.0f32,
    g: 0.5019607843137255f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const OttomanRed: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.13333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const OurLittleSecret: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.29411764705882354f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const OutOfTheBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.6f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Outback: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.6392156862745098f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const OuterSpace: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.3058823529411765f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const OverTheHills: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.42745098039215684f32,
    b: 0.03137254901960784f32,
    a: 1f32,
};
pub const OverTheMoon: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.7215686274509804f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const OverTheSky: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.8352941176470589f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const OverdueBlue: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Overgrown: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const OvergrownCitadel: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const OvergrownMausoleum: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.5333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const OvergrownTemple: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.4f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Overgrowth: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.8f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Oxblood: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.0f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const Oyster: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.8274509803921568f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const OysterIsland: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9372549019607843f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const Pacman: Colour = Colour {
    r: 1.0f32,
    g: 0.9058823529411765f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const Pacific: Colour = Colour {
    r: 0.1411764705882353f32,
    g: 0.39215686274509803f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const PacificBlue: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.6627450980392157f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const PacificDepths: Colour = Colour {
    r: 0.0f32,
    g: 0.26666666666666666f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const PacificNavy: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.2823529411764706f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const PacificPleasure: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.49019607843137253f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const PackingPaper: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.6078431372549019f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const Paella: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.7764705882352941f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const PaidInFull: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.5568627450980392f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const PainterSCanvas: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9490196078431372f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const PaleCanary: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9372549019607843f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const PaleKingSBlue: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.9607843137254902f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const PaleSky: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.9647058823529412f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const PaleWhale: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.8274509803921568f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Palladian: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9137254901960784f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Palm: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.6862745098039216f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const PalmLeaf: Colour = Colour {
    r: 0.21176470588235294f32,
    g: 0.2823529411764706f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const PanPurple: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.47843137254901963f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const Pancake: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8431372549019608f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const PandoraSBox: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8588235294117647f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const Panela: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.3215686274509804f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const PaniPuri: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.6666666666666666f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const Panorama: Colour = Colour {
    r: 0.19607843137254902f32,
    g: 0.47843137254901963f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Pansy: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.3254901960784314f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const Papaya: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.596078431372549f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const PaperHeart: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8588235294117647f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const PaperHearts: Colour = Colour {
    r: 0.8f32,
    g: 0.26666666666666666f32,
    b: 0.4f32,
    a: 1f32,
};
pub const PaperPlane: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9254901960784314f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const PaperWhite: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9411764705882353f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const Paperwhite: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9372549019607843f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Paprika: Colour = Colour {
    r: 0.48627450980392156f32,
    g: 0.17647058823529413f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const PaprikaKisses: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.2627450980392157f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const Papyrus: Colour = Colour {
    r: 0.6f32,
    g: 0.6f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const ParadiseBird: Colour = Colour {
    r: 1.0f32,
    g: 0.5490196078431373f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const ParadiseIsland: Colour = Colour {
    r: 0.35294117647058826f32,
    g: 0.6549019607843137f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const ParadisePink: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.26666666666666666f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const Paradiso: Colour = Colour {
    r: 0.2823529411764706f32,
    g: 0.5019607843137255f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const Parchment: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9882352941176471f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const Parfait: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.6509803921568628f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const ParisPaving: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.4470588235294118f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const ParisianPatina: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.6078431372549019f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const ParmaHam: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.596078431372549f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Parmesan: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const ParsleySprig: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.4392156862745098f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const PartialPink: Colour = Colour {
    r: 1.0f32,
    g: 0.9294117647058824f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const PartyPig: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6f32,
    b: 1.0f32,
    a: 1f32,
};
pub const PartySpongeCake: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8745098039215686f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const PassionFlower: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.33725490196078434f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const PassionForRevenge: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.050980392156862744f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const PassionPlum: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.37254901960784315f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PassionPotion: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.596078431372549f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const PassionatePlum: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.22745098039215686f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Pasta: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8745098039215686f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const PastaLuego: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.8823529411764706f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const PastaRasta: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7686274509803922f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const PastelBlue: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.7490196078431373f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const PastelBrown: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.4117647058823529f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const PastelDay: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8470588235294118f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const PastelDeNata: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.788235294117647f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const PastelGreen: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.8666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PastelGrey: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.8117647058823529f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const PastelLavender: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.6313725490196078f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const PastelLilac: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.6901960784313725f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const PastelMagenta: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.6039215686274509f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const PastelMint: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.9411764705882353f32,
    b: 0.8f32,
    a: 1f32,
};
pub const PastelOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.5882352941176471f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const PastelPea: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.9058823529411765f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const PastelPink: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.6470588235294118f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const PastelPurple: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.6196078431372549f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const PastelRed: Colour = Colour {
    r: 1.0f32,
    g: 0.4117647058823529f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const PastelSmirk: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.9254901960784314f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const PastelTurquoise: Colour = Colour {
    r: 0.6f32,
    g: 0.7725490196078432f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const PastelViolet: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.6f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const PastelYellow: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9921568627450981f32,
    b: 0.5882352941176471f32,
    a: 1f32,
};
pub const Pastrami: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.44313725490196076f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const Pastry: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8705882352941177f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const Patina: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.5725490196078431f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const Patisserie: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8588235294117647f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const Pāua: Colour = Colour {
    r: 0.16470588235294117f32,
    g: 0.1450980392156863f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const PavedWithGold: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8235294117647058f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const PcbGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.17647058823529413f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const Pea: Colour = Colour {
    r: 0.6431372549019608f32,
    g: 0.7490196078431373f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const PeaceAndQuiet: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.8549019607843137f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const Peach: Colour = Colour {
    r: 1.0f32,
    g: 0.6901960784313725f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const PeachBeach: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8117647058823529f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const PeachBeauty: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.7647058823529411f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const PeachBud: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.6980392156862745f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const PeachCream: Colour = Colour {
    r: 1.0f32,
    g: 0.9411764705882353f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const PeachCrèmeBrûlée: Colour = Colour {
    r: 1.0f32,
    g: 0.8823529411764706f32,
    b: 0.615686274509804f32,
    a: 1f32,
};
pub const PeachDunes: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.4117647058823529f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const PeachFizz: Colour = Colour {
    r: 1.0f32,
    g: 0.6588235294117647f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const PeachFury: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.5176470588235295f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const PeachFuzz: Colour = Colour {
    r: 1.0f32,
    g: 0.7803921568627451f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const PeachPuff: Colour = Colour {
    r: 1.0f32,
    g: 0.8549019607843137f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const PeachPunch: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.6f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const PeachScone: Colour = Colour {
    r: 1.0f32,
    g: 0.7372549019607844f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const PeachTaffy: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.7137254901960784f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const PeachVelour: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.6980392156862745f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const PeachSDaydream: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.6078431372549019f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const PeachesOfImmortality: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.5215686274509804f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const PeachyFeeling: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.5254901960784314f32,
    b: 0.4f32,
    a: 1f32,
};
pub const PeachyMilk: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8784313725490196f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const PeachyPinky: Colour = Colour {
    r: 1.0f32,
    g: 0.4666666666666667f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const Peachykini: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.7490196078431373f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const PeacockPride: Colour = Colour {
    r: 0.0f32,
    g: 0.4f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Peanut: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.26666666666666666f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const PeanutButter: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.5372549019607843f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const PeanutButterBiscuit: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.7098039215686275f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const PeanutButterChicken: Colour = Colour {
    r: 1.0f32,
    g: 0.7176470588235294f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const PeanutButterJelly: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.2901960784313726f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const Pear: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.8862745098039215f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Pearl: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8784313725490196f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const PearlBrite: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.9019607843137255f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const PearlPowder: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 1.0f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const PearlWhite: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9490196078431372f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Pearly: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8901960784313725f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const PearlyPink: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6f32,
    b: 0.8f32,
    a: 1f32,
};
pub const PeasPlease: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.4980392156862745f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const PeatBrown: Colour = Colour {
    r: 0.35294117647058826f32,
    g: 0.23921568627450981f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const Pebble: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.596078431372549f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const PebbleBeach: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.5098039215686274f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const Pedigrey: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.5568627450980392f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const PeekABlue: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.8823529411764706f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const Peekaboo: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8705882352941177f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Pegasus: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.9137254901960784f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const Pelati: Colour = Colour {
    r: 1.0f32,
    g: 0.2f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Pelican: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.7372549019607844f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const PepperGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.49019607843137253f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Peppermint: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.9058823529411765f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const Pepperoni: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.26666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const PeppyPeacock: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.8f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const PeppyPineapple: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const PerfectDark: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.2f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const PerfectPink: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.7019607843137254f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const PerfumeCloud: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.788235294117647f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const PerfumeHaze: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9137254901960784f32,
    b: 0.9686274509803922f32,
    a: 1f32,
};
pub const Pergament: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.6470588235294118f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const PeriPeri: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.17647058823529413f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const Periwinkle: Colour = Colour {
    r: 0.5568627450980392f32,
    g: 0.5098039215686274f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const Permafrost: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.9372549019607843f32,
    b: 0.9764705882352941f32,
    a: 1f32,
};
pub const Perrywinkle: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.5490196078431373f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const PersianLuxuryPurple: Colour = Colour {
    r: 0.6f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PersianMelon: Colour = Colour {
    r: 1.0f32,
    g: 0.8627450980392157f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const PersianMosaic: Colour = Colour {
    r: 0.12549019607843137f32,
    g: 0.40784313725490196f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const PersianRed: Colour = Colour {
    r: 0.8f32,
    g: 0.2f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Persicus: Colour = Colour {
    r: 1.0f32,
    g: 0.7058823529411765f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Persimmon: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.6078431372549019f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const Peru: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.5215686274509804f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const PesteringPesto: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.6f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Pesto: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.6980392156862745f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const PestoAllaGenovese: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const PestoDiPistacchio: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.7686274509803922f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const PestoDiRucola: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.5411764705882353f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const PestoRosso: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.2f32,
    b: 0.2f32,
    a: 1f32,
};
pub const PetalOfADyingRose: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.023529411764705882f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const PetalPink: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8980392156862745f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const PetitePink: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.792156862745098f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const Petrified: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.5254901960784314f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const PetrifiedPurple: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.5294117647058824f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const PetrolSlumber: Colour = Colour {
    r: 0.1411764705882353f32,
    g: 0.21176470588235294f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const Phantom: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.4745098039215686f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const PhantomShip: Colour = Colour {
    r: 0.1843137254901961f32,
    g: 0.20392156862745098f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const PharmaceuticalGreen: Colour = Colour {
    r: 0.03137254901960784f32,
    g: 0.49411764705882355f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const PhaserBeam: Colour = Colour {
    r: 1.0f32,
    g: 0.30196078431372547f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Pheasant: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.48627450980392156f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Philodendron: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.38823529411764707f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const PhoenixRed: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.4470588235294118f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const PhosphorGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.6666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const PianoBlack: Colour = Colour {
    r: 0.09019607843137255f32,
    g: 0.09019607843137255f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const Picante: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.28627450980392155f32,
    b: 0.2f32,
    a: 1f32,
};
pub const PiccadillyPurple: Colour = Colour {
    r: 0.3176470588235294f32,
    g: 0.34509803921568627f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const Pickled: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.6549019607843137f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const PickledPineapple: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 1.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const PickledRadish: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const PieCrust: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.8509803921568627f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const PiercingPink: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const PiercingRed: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const PigPink: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8431372549019608f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const Pigeon: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.6862745098039216f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Piggy: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.596078431372549f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const PiggyBank: Colour = Colour {
    r: 1.0f32,
    g: 0.8f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const Piglet: Colour = Colour {
    r: 1.0f32,
    g: 0.7529411764705882f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const PikaYellow: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9137254901960784f32,
    b: 0.17647058823529413f32,
    a: 1f32,
};
pub const Pilsener: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9686274509803922f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const PimentPiquant: Colour = Colour {
    r: 0.8f32,
    g: 0.13333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Pimento: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.36470588235294116f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const PimmS: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.34509803921568627f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const PinaColada: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8705882352941177f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Pinball: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.8274509803921568f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const PinchMe: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.5490196078431373f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const Pine: Colour = Colour {
    r: 0.16862745098039217f32,
    g: 0.36470588235294116f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const PineNeedle: Colour = Colour {
    r: 0.2f32,
    g: 0.30196078431372547f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const PineScentedLagoon: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.43529411764705883f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Pineapple: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.23529411764705882f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const PineapplePerfume: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const PineappleSorbet: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9568627450980393f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const PineappleWhip: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8509803921568627f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Pink: Colour = Colour {
    r: 1.0f32,
    g: 0.7529411764705882f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const PinkBliss: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.6705882352941176f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const PinkBlush: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.6745098039215687f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const PinkChampagne: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8745098039215686f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const PinkElephants: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const PinkFit: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.6588235294117647f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const PinkFlamingo: Colour = Colour {
    r: 1.0f32,
    g: 0.4f32,
    b: 1.0f32,
    a: 1f32,
};
pub const PinkFloyd: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.6039215686274509f32,
    b: 0.615686274509804f32,
    a: 1f32,
};
pub const PinkGlitter: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8745098039215686f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const PinkHorror: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.18823529411764706f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const PinkInk: Colour = Colour {
    r: 1.0f32,
    g: 0.0784313725490196f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const PinkLemonade: Colour = Colour {
    r: 1.0f32,
    g: 0.9176470588235294f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const PinkMacaroon: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.6745098039215687f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const PinkMarshmallow: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.7137254901960784f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const PinkMist: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.7372549019607844f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const PinkOrchid: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.4392156862745098f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const PinkPalazzo: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.6235294117647059f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const PinkPanther: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const PinkParty: Colour = Colour {
    r: 1.0f32,
    g: 0.3333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const PinkPepper: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.34509803921568627f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const PinkPleasure: Colour = Colour {
    r: 1.0f32,
    g: 0.8745098039215686f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const PinkPoison: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const PinkPrestige: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const PinkPride: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.11372549019607843f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const PinkPunk: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.5137254901960784f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const PinkSupremecy: Colour = Colour {
    r: 1.0f32,
    g: 0.8509803921568627f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const Pinkalicious: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Pinkling: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.5176470588235295f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const Pinkman: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.06666666666666667f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Pinky: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.5254901960784314f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const PinkyPickle: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.42745098039215684f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const PinkySwear: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const PinotNoir: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.3215686274509804f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const PirateGold: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.47058823529411764f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const PirateTreasure: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.792156862745098f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const PirateSHook: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.5607843137254902f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const PiscoSour: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.9215686274509803f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const Pistachio: Colour = Colour {
    r: 0.5764705882352941f32,
    g: 0.7725490196078432f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const PistachioShell: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.7725490196078432f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const Pita: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9058823529411765f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const PitchBlack: Colour = Colour {
    r: 0.2823529411764706f32,
    g: 0.23529411764705882f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const PitchblackForests: Colour = Colour {
    r: 0.0f32,
    g: 0.2f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const PixelBleeding: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Pizza: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.5529411764705883f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const PizzaFlame: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.13333333333333333f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const Placebo: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.9058823529411765f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const PlaceboYellow: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.984313725490196f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const PlanetEarth: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.8666666666666667f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const PlanetOfTheApes: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.2f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Plantation: Colour = Colour {
    r: 0.24313725490196078f32,
    g: 0.34901960784313724f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const Plaster: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9176470588235294f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const PlasticCarrot: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.36470588235294116f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const PlasticClouds: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9411764705882353f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const PlasticLips: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.13333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const PlasticVeggie: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 1.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Platinum: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8941176470588236f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const PlatinumBlonde: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9098039215686274f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const PleasantPomegranate: Colour = Colour {
    r: 0.8f32,
    g: 0.2f32,
    b: 0.0f32,
    a: 1f32,
};
pub const PleasantPurple: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.2f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const PleasingPink: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.803921568627451f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const Pleasure: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.2196078431372549f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const PleinAir: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.7686274509803922f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const Plum: Colour = Colour {
    r: 0.4f32,
    g: 0.2196078431372549f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const PlumCheese: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.027450980392156862f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const PlumHighness: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PlumKingdom: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.2f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PlumPerfect: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Plumbeous: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.4470588235294118f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const Plummy: Colour = Colour {
    r: 0.403921568627451f32,
    g: 0.35294117647058826f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const Plunge: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.3333333333333333f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const PlungePool: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.8f32,
    a: 1f32,
};
pub const PlushyPink: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.7176470588235294f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const Plutonium: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.9803921568627451f32,
    b: 0.0f32,
    a: 1f32,
};
pub const PoachedEgg: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.8470588235294118f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const Poblano: Colour = Colour {
    r: 0.027450980392156862f32,
    g: 0.4980392156862745f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const PoisonIvy: Colour = Colour {
    r: 0.0f32,
    g: 0.6784313725490196f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const PoisonPurple: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.00392156862745098f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const PoisonPurpleParadise: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Poisonous: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 1.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const PoisonousDart: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 1.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const PoisonousPesto: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.9098039215686274f32,
    b: 0.0392156862745098f32,
    a: 1f32,
};
pub const PoisonousPistachio: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const PoisonousPotion: Colour = Colour {
    r: 0.6f32,
    g: 0.8666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Polar: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.9490196078431372f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const PolarBear: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9137254901960784f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const PolarBearInABlizzard: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 1.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const PolarExpedition: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.9058823529411765f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const PolarWind: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.8745098039215686f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Polenta: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.7686274509803922f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const PolishedApple: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.16470588235294117f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const PolishedBronze: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.4980392156862745f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const PolishedCopper: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.38823529411764707f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const PolishedGold: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const PolishedLimestone: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.8352941176470589f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const PolishedPearl: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9294117647058824f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const PolishedSilver: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.8196078431372549f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const Pollen: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Pollination: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Polly: Colour = Colour {
    r: 1.0f32,
    g: 0.792156862745098f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const Pomegranate: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.3333333333333333f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const PomeloRed: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.5607843137254902f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const Pomodoro: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.00784313725490196f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const PompeianRed: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.16470588235294117f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const Pompelmo: Colour = Colour {
    r: 1.0f32,
    g: 0.4f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Pony: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.6666666666666666f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const PoodleSkirt: Colour = Colour {
    r: 1.0f32,
    g: 0.6823529411764706f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const PoolTable: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.5843137254901961f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const PoolWater: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.5333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const PopThatGum: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.44313725490196076f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Popcorn: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8156862745098039f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Poppy: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.23529411764705882f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const PoppyFlower: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.34509803921568627f32,
    b: 0.0f32,
    a: 1f32,
};
pub const PoppyRed: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.2196078431372549f32,
    b: 0.27058823529411763f32,
    a: 1f32,
};
pub const Porcelain: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8627450980392157f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const PorcelainMint: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.9058823529411765f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const PorcelainSkin: Colour = Colour {
    r: 1.0f32,
    g: 0.9058823529411765f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const Porcini: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.6823529411764706f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const PorkBelly: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8784313725490196f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Poseidon: Colour = Colour {
    r: 0.0784313725490196f32,
    g: 0.23529411764705882f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const PoseidonJr: Colour = Colour {
    r: 0.4f32,
    g: 0.9333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const PossessedPurple: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const PossessedRed: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.14901960784313725f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const PotOfGold: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.803921568627451f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const PotatoChip: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8627450980392157f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const PowderBlush: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.5803921568627451f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const PowderPuff: Colour = Colour {
    r: 1.0f32,
    g: 0.9372549019607843f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const Powdered: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9490196078431372f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Prairie: Colour = Colour {
    r: 0.043137254901960784f32,
    g: 0.615686274509804f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const PrairieLand: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.8f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const PrairieWinds: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.9019607843137255f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const PraiseTheSun: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9568627450980393f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const Precious: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.8549019607843137f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const PreciousCopper: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const PreciousPersimmon: Colour = Colour {
    r: 1.0f32,
    g: 0.4666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const PreciousPumpkin: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.3843137254901961f32,
    b: 0.2f32,
    a: 1f32,
};
pub const PrehistoricPink: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.45098039215686275f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const PreppyRose: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.4f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const PressingMyLuck: Colour = Colour {
    r: 0.0f32,
    g: 0.8f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const PrettyInPink: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.7490196078431373f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const PrettyInPrune: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.1607843137254902f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const PrettyPastry: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.803921568627451f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const PrettyTwilightNight: Colour = Colour {
    r: 0.1450980392156863f32,
    g: 0.2784313725490196f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const PricklyPink: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.17254901960784313f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const PricklyPurple: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.39215686274509803f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const PrimalGreen: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.5294117647058824f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const PrimalRage: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.18823529411764706f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const Primavera: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.6549019607843137f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Primrose: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.5215686274509804f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const PrinceCharming: Colour = Colour {
    r: 0.8f32,
    g: 0.13333333333333333f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PrincessPeach: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.47058823529411764f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const PrismPink: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.6313725490196078f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Prismarine: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.4666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PristineOceanic: Colour = Colour {
    r: 0.0f32,
    g: 0.8f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const PristineSeas: Colour = Colour {
    r: 0.0f32,
    g: 0.4666666666666667f32,
    b: 0.6f32,
    a: 1f32,
};
pub const ProfessorPlum: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.20784313725490197f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const PromCorsage: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.7647058823529411f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const PromQueen: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.11372549019607843f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Promenade: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9647058823529412f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const PrometheusOrange: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.34509803921568627f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const PromiscuousPink: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const ProphetViolet: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.34509803921568627f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const Prosciutto: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.7058823529411765f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const Prosecco: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.8392156862745098f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const Prune: Colour = Colour {
    r: 0.4392156862745098f32,
    g: 0.10980392156862745f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Prunella: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.2784313725490196f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const PsychedelicPurple: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const PuckerUp: Colour = Colour {
    r: 1.0f32,
    g: 0.06666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const PuffOfPink: Colour = Colour {
    r: 1.0f32,
    g: 0.796078431372549f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const PuffyCloud: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.8705882352941177f32,
    b: 0.9490196078431372f32,
    a: 1f32,
};
pub const PuffyPillow: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8980392156862745f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const Pulp: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.5098039215686274f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const Puma: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.44313725490196076f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const PumpingSpice: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.3137254901960784f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Pumpkin: Colour = Colour {
    r: 1.0f32,
    g: 0.4588235294117647f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const PumpkinCat: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.4823529411764706f32,
    b: 0.027450980392156862f32,
    a: 1f32,
};
pub const PumpkinPie: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.6196078431372549f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const Punch: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.2627450980392157f32,
    b: 0.2f32,
    a: 1f32,
};
pub const PunkRockPurple: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const PureBlue: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.011764705882352941f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const PurePassion: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.0f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const PurePleasure: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.07450980392156863f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const PureSunshine: Colour = Colour {
    r: 1.0f32,
    g: 0.9333333333333333f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const Purple: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.0f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const PurpleClimax: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const PurpleEmperor: Colour = Colour {
    r: 0.4f32,
    g: 0.2f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const PurpleExcellency: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.20784313725490197f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const PurpleHaze: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.45098039215686275f32,
    b: 0.5882352941176471f32,
    a: 1f32,
};
pub const PurpleHeart: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.20784313725490197f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const PurpleIllusion: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.7215686274509804f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const PurpleInk: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.17254901960784313f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const PurpleNoir: Colour = Colour {
    r: 0.19607843137254902f32,
    g: 0.17254901960784313f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const PurplePassion: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.27450980392156865f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const PurplePirate: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const PurplePizzazz: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.3058823529411765f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const PurplePleasures: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.27058823529411763f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const PurplePoodle: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.7058823529411765f32,
    b: 0.8f32,
    a: 1f32,
};
pub const PurplePristine: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.2f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const PurpleProse: Colour = Colour {
    r: 0.32941176470588235f32,
    g: 0.19607843137254902f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const PurpleProtégé: Colour = Colour {
    r: 0.34901960784313724f32,
    g: 0.20784313725490197f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const PurpleRain: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.25882352941176473f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const PurpleSultan: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.21176470588235294f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const PurpleVelour: Colour = Colour {
    r: 0.34509803921568627f32,
    g: 0.10196078431372549f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const PurpleVoid: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.13333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const PurpleSBabySister: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7647058823529411f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const PutOnIce: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.8666666666666667f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const Pyramid: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.49019607843137253f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const QuackQuack: Colour = Colour {
    r: 1.0f32,
    g: 0.9137254901960784f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const QueenBlue: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.4196078431372549f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const QueenOfGardens: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const QueenOfHearts: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.2f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const QueenOfTrees: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.25098039215686274f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const QueerBlue: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.6745098039215687f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const QuicheLorraine: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8352941176470589f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const Quickfreeze: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.8588235294117647f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const Quicksand: Colour = Colour {
    r: 0.6745098039215687f32,
    g: 0.596078431372549f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const Quicksilver: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.6509803921568628f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const QuietAbyss: Colour = Colour {
    r: 0.08627450980392157f32,
    g: 0.01568627450980392f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const QuietHarbour: Colour = Colour {
    r: 0.35294117647058826f32,
    g: 0.47058823529411764f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const QuillGrey: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.788235294117647f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const Quince: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.796078431372549f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const RaceTheSun: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9529411764705882f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const RacingRed: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.09019607843137255f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const RadiantFoliage: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.611764705882353f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const RadiantHulk: Colour = Colour {
    r: 0.06274509803921569f32,
    g: 0.9450980392156862f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const RadiantRaspberry: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.10588235294117647f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const RadiantSunrise: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7450980392156863f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const RadicalRed: Colour = Colour {
    r: 1.0f32,
    g: 0.20784313725490197f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const Radioactive: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.996078431372549f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const Radish: Colour = Colour {
    r: 0.6431372549019608f32,
    g: 0.1803921568627451f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const Radishical: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.2823529411764706f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const Rage: Colour = Colour {
    r: 1.0f32,
    g: 0.06666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const RagingRaisin: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.2f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Rainforest: Colour = Colour {
    r: 0.0f32,
    g: 0.6039215686274509f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const RainyMood: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.6f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Rajah: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.6705882352941176f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const RampantRhubarb: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.19607843137254902f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Rampart: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.7176470588235294f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const RanchHouse: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.39215686274509803f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const Rapeseed: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.6039215686274509f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const RaptureSLight: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9529411764705882f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const RapunzelSilver: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.8235294117647058f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const RareBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.26666666666666666f32,
    b: 1.0f32,
    a: 1f32,
};
pub const RareRed: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Raspberry: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.00392156862745098f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const RaspberryMousse: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.43529411764705883f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const RaspberryRomantic: Colour = Colour {
    r: 0.592156862745098f32,
    g: 0.16862745098039217f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const Raven: Colour = Colour {
    r: 0.043137254901960784f32,
    g: 0.043137254901960784f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const RavenSCoat: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.00784313725490196f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const Ravenclaw: Colour = Colour {
    r: 0.0392156862745098f32,
    g: 0.0196078431372549f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const RavioliAlLimone: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.8705882352941177f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const RazzleDazzle: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.2549019607843137f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const ReentryRed: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.011764705882352941f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const ReadingTeaLeaves: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.36470588235294116f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const RealmOfTheUnderworld: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.26666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const RebellionRed: Colour = Colour {
    r: 0.8f32,
    g: 0.01568627450980392f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const Red: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const RedAlert: Colour = Colour {
    r: 1.0f32,
    g: 0.058823529411764705f32,
    b: 0.058823529411764705f32,
    a: 1f32,
};
pub const RedArremer: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.3058823529411765f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const RedBaron: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const RedCarpet: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.12549019607843137f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const RedCent: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.396078431372549f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const RedDevil: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.00392156862745098f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const RedElegance: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.27450980392156865f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const RedFlag: Colour = Colour {
    r: 1.0f32,
    g: 0.13333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const RedHerring: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const RedHotChiliPepper: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.11372549019607843f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const RedInferno: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.11764705882352941f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const RedMana: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.3333333333333333f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const RedMenace: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.12941176470588237f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const RedMyMind: Colour = Colour {
    r: 0.6f32,
    g: 0.2627450980392157f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const RedOctopus: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.19607843137254902f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const RedPanda: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.29411764705882354f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const RedPegasus: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const RedRadish: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.2f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const RedReign: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.027450980392156862f32,
    b: 0.027450980392156862f32,
    a: 1f32,
};
pub const RedRepublic: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.00784313725490196f32,
    b: 0.0f32,
    a: 1f32,
};
pub const RedRibbon: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.0392156862745098f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const RedRidingHood: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.15294117647058825f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const RedRobin: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.2549019607843137f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const RedStop: Colour = Colour {
    r: 1.0f32,
    g: 0.13333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const RedTape: Colour = Colour {
    r: 0.8f32,
    g: 0.06666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const RedWrathOfZeus: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.09411764705882353f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const Redolency: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.5411764705882353f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Redstone: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.4196078431372549f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const Redsurrection: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.043137254901960784f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const Redяum: Colour = Colour {
    r: 1.0f32,
    g: 0.13333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const ReignOfTomatoes: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.1450980392156863f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const ReptileRevenge: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.34509803921568627f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const ResplendentGrowth: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.5450980392156862f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const RestfulRain: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9490196078431372f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const RetroNectarine: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.49019607843137253f32,
    b: 0.08627450980392157f32,
    a: 1f32,
};
pub const RetroPinkPop: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const RetroVibe: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.592156862745098f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const RichGold: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.5333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Ridgeback: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.596078431372549f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const Ripasso: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.19215686274509805f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const RipeMalinka: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.3411764705882353f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const RisingStar: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.9647058823529412f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const Riverbed: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.7450980392156863f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const Roasted: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.3215686274509804f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const RoastedPepper: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.0392156862745098f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const Roastery: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.13725490196078433f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const RoboticGods: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.6352941176470588f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const RockLobster: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.043137254901960784f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const RockNRose: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.5411764705882353f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Rolandgarros: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const RomanicScene: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.011764705882352941f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const RomanticEmbers: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.24313725490196078f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const RomanticThriller: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.06274509803921569f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const RomanticVampire: Colour = Colour {
    r: 0.6f32,
    g: 0.06666666666666667f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Romesco: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.5058823529411764f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const RooftopGarden: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.6784313725490196f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const RootBeer: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.32941176470588235f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Rosé: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.4549019607843137f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const RoseAshes: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.6745098039215687f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const RoseElegance: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.6313725490196078f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const RoseLaffyTaffy: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.27450980392156865f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const RoseateSpoonbill: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.6784313725490196f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const Rosecco: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Rosemarried: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.6078431372549019f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const Rosemary: Colour = Colour {
    r: 0.25098039215686274f32,
    g: 0.3686274509803922f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const RosesAreRed: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.21176470588235294f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const RosesInTheSnow: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.6823529411764706f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Rosetti: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.5725490196078431f32,
    b: 0.6039215686274509f32,
    a: 1f32,
};
pub const Rosewood: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.0f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const RosewoodDreams: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.7450980392156863f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const RosyBrown: Colour = Colour {
    r: 0.7372549019607844f32,
    g: 0.5607843137254902f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const Rouge: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.07058823529411765f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const RoughAsphalt: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.7450980392156863f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Royal: Colour = Colour {
    r: 0.047058823529411764f32,
    g: 0.09019607843137255f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const RoyalDecree: Colour = Colour {
    r: 0.25098039215686274f32,
    g: 0.20784313725490197f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const RoyalLavender: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.3176470588235294f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const RoyalMilkTea: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8117647058823529f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const RoyalNeptune: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.23137254901960785f32,
    b: 0.25882352941176473f32,
    a: 1f32,
};
pub const RoyalNight: Colour = Colour {
    r: 0.16862745098039217f32,
    g: 0.19215686274509805f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const RoyalPlum: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.2549019607843137f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const RoyalPurpleness: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const RoyalRobe: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.2901960784313726f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const RoyalStar: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8705882352941177f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const Royalty: Colour = Colour {
    r: 0.34901960784313724f32,
    g: 0.18823529411764706f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const RubberDucky: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.8117647058823529f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Ruby: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.00392156862745098f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const RubyQueen: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.023529411764705882f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const Rubylicious: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.0784313725490196f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const RuinedSmores: Colour = Colour {
    r: 0.058823529411764705f32,
    g: 0.06274509803921569f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const RuinsOfCivilization: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.8705882352941177f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const RuinsOfMetal: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.5450980392156862f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const Rum: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.4f32,
    b: 0.4588235294117647f32,
    a: 1f32,
};
pub const RunLolaRun: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.1568627450980392f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const RuralGreen: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.5176470588235295f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const RuralRed: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Rust: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.23529411764705882f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const RusticRouge: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.14901960784313725f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const RustlingLeaves: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.4117647058823529f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const RustyHeart: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.25098039215686274f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const RustyRed: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.1843137254901961f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const RuthlessEmpress: Colour = Colour {
    r: 0.3411764705882353f32,
    g: 0.2196078431372549f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const Sablé: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.8470588235294118f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const SableCloaked: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.6549019607843137f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const SacredScarlet: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.047058823529411764f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const SacrificeAltar: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.00392156862745098f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const SacroBosco: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.6f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const SaddleUp: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.5725490196078431f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const SafetyOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.4f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Safflower: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.6823529411764706f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Saffron: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.7686274509803922f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const SaffronDesires: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.2627450980392157f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const Sage: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.6823529411764706f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const SageSensation: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.8823529411764706f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const SailIntoTheHorizon: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.7333333333333333f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const SailOn: Colour = Colour {
    r: 0.27058823529411763f32,
    g: 0.4588235294117647f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const SailToTheSea: Colour = Colour {
    r: 0.6f32,
    g: 0.7647058823529411f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const Sailor: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.3411764705882353f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const SailorBoy: Colour = Colour {
    r: 0.6823529411764706f32,
    g: 0.7333333333333333f32,
    b: 0.8156862745098039f32,
    a: 1f32,
};
pub const SailorMoon: Colour = Colour {
    r: 1.0f32,
    g: 0.9333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Sakura: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.6941176470588235f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const SakuraNight: Colour = Colour {
    r: 0.4823529411764706f32,
    g: 0.4235294117647059f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const Salametti: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.3686274509803922f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Salami: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Salmon: Colour = Colour {
    r: 1.0f32,
    g: 0.4745098039215686f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const SalmonFlush: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.788235294117647f32,
    b: 0.8f32,
    a: 1f32,
};
pub const SalmonGlow: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.7254901960784313f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const SalmonNigiri: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.5647058823529412f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const SalmonPokéBowl: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.4666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const SalmonSashimi: Colour = Colour {
    r: 1.0f32,
    g: 0.49411764705882355f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const SalsaPicante: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.1450980392156863f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const SalsaVerde: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.7803921568627451f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Salt: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9294117647058824f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const SaltCaramel: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.5764705882352941f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const SaltMountain: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.996078431372549f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const SaltNPepa: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.8509803921568627f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const Salted: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.9176470588235294f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const SaltedCapers: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.5686274509803921f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const SaltedCaramelPopcorn: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.6980392156862745f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const SaltyIce: Colour = Colour {
    r: 0.8f32,
    g: 0.8862745098039215f32,
    b: 0.9529411764705882f32,
    a: 1f32,
};
pub const Salvia: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.7098039215686275f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const Samba: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.14901960784313725f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const Sand: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.792156862745098f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const SandDune: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.8235294117647058f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const SandRipples: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.7176470588235294f32,
    b: 0.6901960784313725f32,
    a: 1f32,
};
pub const Sandstorm: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.8352941176470589f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const Sandworm: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9098039215686274f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const Sanguinary: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.10196078431372549f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const Sanguine: Colour = Colour {
    r: 0.4235294117647059f32,
    g: 0.06666666666666667f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const Sappanwood: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.23921568627450981f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const SapphireGlitter: Colour = Colour {
    r: 0.0f32,
    g: 0.2f32,
    b: 0.8f32,
    a: 1f32,
};
pub const SapphireSiren: Colour = Colour {
    r: 0.4f32,
    g: 0.13333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const SapphireSplendour: Colour = Colour {
    r: 0.1411764705882353f32,
    g: 0.1450980392156863f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const SassyLime: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8862745098039215f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const SassySalmon: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.48627450980392156f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const SatinChocolate: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.2f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const SatinCreamWhite: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9529411764705882f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const SatinDeepBlack: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.11764705882352941f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const SatinLime: Colour = Colour {
    r: 0.2f32,
    g: 0.9333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Saturn: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.8980392156862745f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Sauerkraut: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8784313725490196f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const SaunaSteam: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9215686274509803f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const SavannahGrass: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.7372549019607844f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const SavorySalmon: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.611764705882353f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const SavoyBlue: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.3803921568627451f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const Scampi: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.38823529411764707f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const ScandinavianLiquorice: Colour = Colour {
    r: 0.10196078431372549f32,
    g: 0.06666666666666667f32,
    b: 0.06274509803921569f32,
    a: 1f32,
};
pub const Scarlet: Colour = Colour {
    r: 1.0f32,
    g: 0.1411764705882353f32,
    b: 0.0f32,
    a: 1f32,
};
pub const ScarletBlaze: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.12156862745098039f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const ScarletGlow: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.00392156862745098f32,
    b: 0.011764705882352941f32,
    a: 1f32,
};
pub const ScarletSplendour: Colour = Colour {
    r: 0.8f32,
    g: 0.047058823529411764f32,
    b: 0.10588235294117647f32,
    a: 1f32,
};
pub const ScentedSpring: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8352941176470589f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const SchiaparelliPink: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.28627450980392155f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const SchoolBus: Colour = Colour {
    r: 1.0f32,
    g: 0.8470588235294118f32,
    b: 0.0f32,
    a: 1f32,
};
pub const ScovilleHighness: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.01568627450980392f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const ScreaminGreen: Colour = Colour {
    r: 0.4f32,
    g: 1.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Sea: Colour = Colour {
    r: 0.23529411764705882f32,
    g: 0.6f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const SeaCreature: Colour = Colour {
    r: 0.0f32,
    g: 0.34509803921568627f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const SeaFoam: Colour = Colour {
    r: 0.5294117647058824f32,
    g: 0.8784313725490196f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const SeaFoamMist: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.8627450980392157f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const SeaGlassTeal: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.8980392156862745f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const SeaGoddess: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.4117647058823529f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const SeaLion: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.5294117647058824f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const SeaOfGalilee: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.396078431372549f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const SeaPaint: Colour = Colour {
    r: 0.0f32,
    g: 0.3137254901960784f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const SeaSaltRivers: Colour = Colour {
    r: 0.3137254901960784f32,
    g: 0.5294117647058824f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const SeaSerpent: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.7803921568627451f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const SeaSerpentSTears: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Seaborn: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.7607843137254902f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const Seafarer: Colour = Colour {
    r: 0.12549019607843137f32,
    g: 0.30196078431372547f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const SeafoamSlate: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.7372549019607844f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const SeafoamSplashes: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.9372549019607843f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const SeafoamWhisper: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.7411764705882353f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Seashell: Colour = Colour {
    r: 1.0f32,
    g: 0.9607843137254902f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Seaside: Colour = Colour {
    r: 0.4f32,
    g: 0.6431372549019608f32,
    b: 0.6901960784313725f32,
    a: 1f32,
};
pub const Seaweed: Colour = Colour {
    r: 0.09411764705882353f32,
    g: 0.8196078431372549f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const SecretAffair: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.08627450980392157f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const SecretBlush: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8235294117647058f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const SecretGarden: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.6666666666666666f32,
    b: 0.4f32,
    a: 1f32,
};
pub const SecretOfMana: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.4f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const SecretPassage: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.16470588235294117f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const SecretPath: Colour = Colour {
    r: 0.45098039215686275f32,
    g: 0.4392156862745098f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const SecretScarlet: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.054901960784313725f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const Seedling: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.796078431372549f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const Selflove: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.16862745098039217f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const Semolina: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.6705882352941176f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const Serene: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.8901960784313725f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const SereneSea: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.6549019607843137f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const SereniTeal: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.7294117647058823f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const SerenitySReign: Colour = Colour {
    r: 0.3137254901960784f32,
    g: 0.4823529411764706f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const SerialKisses: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.21568627450980393f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const SeriousCloud: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.5176470588235295f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const SerpentScepter: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.8f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SerranoPepper: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.4f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Sesame: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.6392156862745098f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const SesameSeed: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8509803921568627f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const SevenSeas: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.3607843137254902f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const ShadesOfRuby: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.0f32,
    b: 0.03529411764705882f32,
    a: 1f32,
};
pub const ShadowOfNight: Colour = Colour {
    r: 0.16470588235294117f32,
    g: 0.30980392156862746f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const ShadowOfTheColossus: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.6352941176470588f32,
    b: 0.6313725490196078f32,
    a: 1f32,
};
pub const ShadowPurple: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.2f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const Shadows: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.42745098039215684f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const ShadyCharacter: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.29411764705882354f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const ShallowSea: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.7215686274509804f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const ShallowWater: Colour = Colour {
    r: 0.5411764705882353f32,
    g: 0.9450980392156862f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const Shamrock: Colour = Colour {
    r: 0.0f32,
    g: 0.6196078431372549f32,
    b: 0.3764705882352941f32,
    a: 1f32,
};
pub const Shark: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.8627450980392157f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const ShatteredIce: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.9333333333333333f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const ShavingCream: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8980392156862745f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const SheLovesPink: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.6078431372549019f32,
    b: 0.5882352941176471f32,
    a: 1f32,
};
pub const Sheaf: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.6823529411764706f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const SheikhWhite: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9254901960784314f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Shiitake: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.596078431372549f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const ShimmeringBlue: Colour = Colour {
    r: 0.5098039215686274f32,
    g: 0.8588235294117647f32,
    b: 0.8f32,
    a: 1f32,
};
pub const ShimmeringLove: Colour = Colour {
    r: 1.0f32,
    g: 0.5333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const ShinGodzilla: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.21568627450980393f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const ShinyTrumpet: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.6823529411764706f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Shipmate: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.6392156862745098f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Shipwreck: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.5294117647058824f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const Shipyard: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.43529411764705883f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const ShockingCrimson: Colour = Colour {
    r: 1.0f32,
    g: 0.050980392156862744f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const ShockingOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.43137254901960786f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const ShockingPink: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.00784313725490196f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const Shōji: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8352941176470589f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Shortbread: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9019607843137255f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const Shrimp: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.6039215686274509f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const ShrimpCocktail: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.6431372549019608f32,
    b: 0.3803921568627451f32,
    a: 1f32,
};
pub const ShrimpToast: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.7725490196078432f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const ShrineOfPleasures: Colour = Colour {
    r: 0.8f32,
    g: 0.2f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Shuriken: Colour = Colour {
    r: 0.2f32,
    g: 0.2f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const ShyChampagneBlush: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.6392156862745098f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const ShyYoungSalmon: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.7215686274509804f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const Sienna: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.33725490196078434f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const SignalGreen: Colour = Colour {
    r: 0.2f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Silence: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9294117647058824f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const SilentFilm: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.6470588235294118f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const SilentNight: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.403921568627451f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const SilentSea: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.2901960784313726f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const SilkDessou: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9137254901960784f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const SilkForTheGods: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.8666666666666667f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const SilkLining: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9372549019607843f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const SilkSatin: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.25882352941176473f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const SilkStar: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9333333333333333f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const SilkenChocolate: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.49019607843137253f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const SilkenGold: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8823529411764706f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const SilkenJade: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.6392156862745098f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const SilkenPebble: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.8156862745098039f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const SilkenRuby: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.07450980392156863f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const Silkworm: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const SilkyGreen: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.7607843137254902f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const SilkyMint: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.9254901960784314f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const SilkyWhite: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9215686274509803f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const Silver: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.7529411764705882f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const SilverBirch: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.8117647058823529f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const SilverBird: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9607843137254902f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const SilverFern: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8666666666666667f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const SilverFox: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.7372549019607844f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const SilverLake: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8666666666666667f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const SilverLining: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.6941176470588235f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const SilverMistral: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.7254901960784313f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const SilverPhoenix: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.9254901960784314f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const SilverSurfer: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.49019607843137253f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Silvertongued: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.7803921568627451f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Silverback: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.796078431372549f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const Silverfish: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.5843137254901961f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Silverplate: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.7529411764705882f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const SimplyPurple: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.3568627450980392f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const SingleOrigin: Colour = Colour {
    r: 0.44313725490196076f32,
    g: 0.24313725490196078f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const Sinister: Colour = Colour {
    r: 0.07058823529411765f32,
    g: 0.06666666666666667f32,
    b: 0.054901960784313725f32,
    a: 1f32,
};
pub const Sinsemilla: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.7411764705882353f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const SipOfMint: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8745098039215686f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const Siren: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.1607843137254902f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const SirenScarlet: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.11372549019607843f32,
    b: 0.11372549019607843f32,
    a: 1f32,
};
pub const SizzlingWatermelon: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.0f32,
    b: 0.3607843137254902f32,
    a: 1f32,
};
pub const Skeleton: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8705882352941177f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Sky: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.8392156862745098f32,
    b: 1.0f32,
    a: 1f32,
};
pub const SkyDancer: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.6f32,
    b: 1.0f32,
    a: 1f32,
};
pub const SkyDive: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.7490196078431373f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const SkyFall: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.7764705882352941f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const SkyHigh: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.7607843137254902f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const Skydiver: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.6745098039215687f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const Skyscraper: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.8588235294117647f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const Skyvory: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.8431372549019608f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const SleeplessBlue: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.8588235294117647f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const SleepyHollows: Colour = Colour {
    r: 0.5137254901960784f32,
    g: 0.611764705882353f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const SliceOfHeaven: Colour = Colour {
    r: 0.0f32,
    g: 0.13333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const SlightlyInLove: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9019607843137255f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const SlimeGirl: Colour = Colour {
    r: 0.0f32,
    g: 0.7333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const SlipperSatin: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.7568627450980392f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const SlipperySalmon: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.49411764705882355f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const SlipperySoap: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9294117647058824f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const Slumber: Colour = Colour {
    r: 0.17647058823529413f32,
    g: 0.3176470588235294f32,
    b: 0.48627450980392156f32,
    a: 1f32,
};
pub const SlyFox: Colour = Colour {
    r: 0.5019607843137255f32,
    g: 0.2784313725490196f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const Smalt: Colour = Colour {
    r: 0.0f32,
    g: 0.2f32,
    b: 0.6f32,
    a: 1f32,
};
pub const SmashingPumpkins: Colour = Colour {
    r: 1.0f32,
    g: 0.3333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const SmellOfGarlic: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.8666666666666667f32,
    b: 0.796078431372549f32,
    a: 1f32,
};
pub const SmellTheMint: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.9686274509803922f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const SmidgenOfLove: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const SmileyFace: Colour = Colour {
    r: 1.0f32,
    g: 0.788235294117647f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const SmokeAndMirrors: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.9019607843137255f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const SmokeDragon: Colour = Colour {
    r: 0.8f32,
    g: 0.7333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const SmokedBlackCoffee: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.1843137254901961f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const SmokedOyster: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.8235294117647058f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const SmokedSalmon: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.5019607843137255f32,
    b: 0.4470588235294118f32,
    a: 1f32,
};
pub const Smokescreen: Colour = Colour {
    r: 0.3686274509803922f32,
    g: 0.3411764705882353f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const SmokingMirror: Colour = Colour {
    r: 0.6352941176470588f32,
    g: 0.5843137254901961f32,
    b: 0.5294117647058824f32,
    a: 1f32,
};
pub const SmokyStudio: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.5215686274509804f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const SmoochRouge: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.23921568627450981f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const SmoothPebbles: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7294117647058823f32,
    b: 0.6941176470588235f32,
    a: 1f32,
};
pub const SmoulderingRed: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.20392156862745098f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const SnakeFruit: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.13333333333333333f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const SnakesInTheGrass: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.592156862745098f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const SnarkyMint: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.8901960784313725f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const SneakyDevil: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.0f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const SneakySesame: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.41568627450980394f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const Snow: Colour = Colour {
    r: 1.0f32,
    g: 0.9803921568627451f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const SnowWhite: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 1.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const Snowflake: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9411764705882353f32,
    b: 0.9411764705882353f32,
    a: 1f32,
};
pub const Snowman: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9803921568627451f32,
    b: 0.984313725490196f32,
    a: 1f32,
};
pub const SnowyMint: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.9411764705882353f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const SnowySummit: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.8470588235294118f32,
    b: 0.9137254901960784f32,
    a: 1f32,
};
pub const SoSour: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const SoakedInSun: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.8196078431372549f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Soba: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.7058823529411765f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const Socialist: Colour = Colour {
    r: 0.5725490196078431f32,
    g: 0.10196078431372549f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const SodaPop: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.7764705882352941f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const SoftBlush: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.7372549019607844f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const SoftBoiled: Colour = Colour {
    r: 1.0f32,
    g: 0.7176470588235294f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const SoftButter: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8823529411764706f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const SoftCashmere: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.7137254901960784f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const SoftPillow: Colour = Colour {
    r: 1.0f32,
    g: 0.9607843137254902f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const SoftPumpkin: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.5568627450980392f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Solar: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9176470588235294f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const SolarAsh: Colour = Colour {
    r: 0.8f32,
    g: 0.4f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const SolarFlare: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.48627450980392156f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const SolarPower: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.7058823529411765f32,
    b: 0.20784313725490197f32,
    a: 1f32,
};
pub const SolarStorm: Colour = Colour {
    r: 1.0f32,
    g: 0.7568627450980392f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Sombrero: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.611764705882353f32,
    b: 0.5490196078431373f32,
    a: 1f32,
};
pub const SomewhereInAFairytale: Colour = Colour {
    r: 0.8f32,
    g: 0.6f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const SoothingSapphire: Colour = Colour {
    r: 0.18823529411764706f32,
    g: 0.49019607843137253f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const Sooty: Colour = Colour {
    r: 0.0784313725490196f32,
    g: 0.0784313725490196f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const SorrenoLemon: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.8156862745098039f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Soufflé: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8196078431372549f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const Soulless: Colour = Colour {
    r: 0.10588235294117647f32,
    g: 0.08235294117647059f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const Sour: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.9294117647058824f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const SourAppleCandy: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.9333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const SourAppleRings: Colour = Colour {
    r: 0.2f32,
    g: 0.7333333333333333f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SourCherry: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.2784313725490196f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const SourGreen: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.9019607843137255f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const SourLemon: Colour = Colour {
    r: 1.0f32,
    g: 0.9333333333333333f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const SourYellow: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 1.0f32,
    b: 0.01568627450980392f32,
    a: 1f32,
};
pub const SovereignRed: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.1411764705882353f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const SovietGold: Colour = Colour {
    r: 1.0f32,
    g: 0.8509803921568627f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SoyMilk: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.8235294117647058f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Spa: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.9254901960784314f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const SpaceBattleBlue: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.0f32,
    b: 0.6f32,
    a: 1f32,
};
pub const SpaceColonization: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.058823529411764705f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const SpaceDust: Colour = Colour {
    r: 0.0f32,
    g: 0.13333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const SpaceExploration: Colour = Colour {
    r: 0.0f32,
    g: 0.06666666666666667f32,
    b: 0.6f32,
    a: 1f32,
};
pub const SpaceMissions: Colour = Colour {
    r: 0.19607843137254902f32,
    g: 0.26666666666666666f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const SpaceOpera: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const SpaghettiMonster: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const SparklingChampagne: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8117647058823529f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const SparklingCider: Colour = Colour {
    r: 1.0f32,
    g: 0.9921568627450981f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const SparklingCosmo: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.45098039215686275f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const SparklingSnow: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.996078431372549f32,
    b: 0.9921568627450981f32,
    a: 1f32,
};
pub const SparkyBlue: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.9333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const SpätzleYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.9333333333333333f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Spearmint: Colour = Colour {
    r: 0.39215686274509803f32,
    g: 0.7490196078431373f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const SpectacularPurple: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.00784313725490196f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const SpectacularSaffron: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8509803921568627f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const SpectacularScarlet: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.13725490196078433f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const Sphinx: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.5843137254901961f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const SpiceMarket: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.2823529411764706f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const Spiced: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.44313725490196076f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const SpicedUpOrange: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.47843137254901963f32,
    b: 0.21568627450980393f32,
    a: 1f32,
};
pub const SpicyBerry: Colour = Colour {
    r: 0.8f32,
    g: 0.2f32,
    b: 0.4f32,
    a: 1f32,
};
pub const SpicyCinnamon: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.33725490196078434f32,
    b: 0.1411764705882353f32,
    a: 1f32,
};
pub const SpicyPaella: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.5607843137254902f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const SpicyPurple: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.2235294117647059f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const SpicySweetcorn: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.6745098039215687f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SpikeyRed: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Splashdown: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.9098039215686274f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const SplashingWave: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.8666666666666667f32,
    b: 1.0f32,
    a: 1f32,
};
pub const SplatterMovie: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.10196078431372549f32,
    b: 0.17254901960784313f32,
    a: 1f32,
};
pub const SpringBud: Colour = Colour {
    r: 0.6549019607843137f32,
    g: 0.9882352941176471f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SpringForth: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.7333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const SpringtideMelodies: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.6627450980392157f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const SprinkledWithPink: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.6352941176470588f32,
    b: 0.6823529411764706f32,
    a: 1f32,
};
pub const Sprouted: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8313725490196079f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const StainlessSteel: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.7411764705882353f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const Star: Colour = Colour {
    r: 1.0f32,
    g: 0.8980392156862745f32,
    b: 0.0f32,
    a: 1f32,
};
pub const StarDust: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9529411764705882f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const StarOfLife: Colour = Colour {
    r: 0.0196078431372549f32,
    g: 0.4823529411764706f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const StardustEvening: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.7490196078431373f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const Stargazing: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.27058823529411763f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const Starlet: Colour = Colour {
    r: 0.5215686274509804f32,
    g: 0.3058823529411765f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const StarlitNight: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.2784313725490196f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const StarryNight: Colour = Colour {
    r: 0.1568627450980392f32,
    g: 0.39215686274509803f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const StarshipTonic: Colour = Colour {
    r: 0.8f32,
    g: 0.9058823529411765f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const Starstruck: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.39215686274509803f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const Statuary: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.6431372549019608f32,
    b: 0.6470588235294118f32,
    a: 1f32,
};
pub const StayTheNight: Colour = Colour {
    r: 0.19215686274509805f32,
    g: 0.27450980392156865f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const Steam: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8666666666666667f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const SteamBath: Colour = Colour {
    r: 0.8f32,
    g: 0.8156862745098039f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const SteamEngine: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.6980392156862745f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const SteampunkGold: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.611764705882353f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const SteamyDumpling: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9137254901960784f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const SteelMist: Colour = Colour {
    r: 0.7764705882352941f32,
    g: 0.807843137254902f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const SteelyGrey: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.592156862745098f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Stellar: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.39215686274509803f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const StereotypicalDuck: Colour = Colour {
    r: 1.0f32,
    g: 0.9607843137254902f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const Sterling: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.8313725490196079f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const StillWater: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.36470588235294116f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const StoneCold: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.3333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const StoneFortress: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.7529411764705882f32,
    b: 0.6901960784313725f32,
    a: 1f32,
};
pub const StoneGuardians: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7294117647058823f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const Stop: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.22745098039215686f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const Stoplight: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const Storm: Colour = Colour {
    r: 0.0f32,
    g: 0.043137254901960784f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const StormIsComing: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.23921568627450981f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Stormy: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.7372549019607844f32,
    b: 0.7647058823529411f32,
    a: 1f32,
};
pub const StormyBay: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.6862745098039216f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const StormyHorizon: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.4666666666666667f32,
    b: 0.6f32,
    a: 1f32,
};
pub const StormyNight: Colour = Colour {
    r: 0.21568627450980393f32,
    g: 0.13725490196078433f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const StormyOceans: Colour = Colour {
    r: 0.4392156862745098f32,
    g: 0.5058823529411764f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const StormyPassion: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.4f32,
    b: 0.4f32,
    a: 1f32,
};
pub const StormySea: Colour = Colour {
    r: 0.43137254901960786f32,
    g: 0.5019607843137255f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const StormyWaters: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.6627450980392157f32,
    b: 0.6901960784313725f32,
    a: 1f32,
};
pub const StrawGold: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9647058823529412f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const Strawberry: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.1607843137254902f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const StrawberryAvalanche: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.30980392156862746f32,
    b: 0.2549019607843137f32,
    a: 1f32,
};
pub const StrawberryBlonde: Colour = Colour {
    r: 1.0f32,
    g: 0.8549019607843137f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const StrawberryBonbon: Colour = Colour {
    r: 1.0f32,
    g: 0.9215686274509803f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const StrawberryButtercream: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.7019607843137254f32,
    b: 1.0f32,
    a: 1f32,
};
pub const StrawberryDreams: Colour = Colour {
    r: 1.0f32,
    g: 0.5333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const StrawberryField: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.5137254901960784f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const StrawberryFrappé: Colour = Colour {
    r: 1.0f32,
    g: 0.6352941176470588f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const StrawberryMilk: Colour = Colour {
    r: 1.0f32,
    g: 0.8509803921568627f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const StrawberryMilkshake: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.44313725490196076f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const StrawberryMoon: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.3333333333333333f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const StrawberryRipple: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.803921568627451f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const StrawberryRisotto: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7764705882352941f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Stroopwafel: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.43529411764705883f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const Submerged: Colour = Colour {
    r: 0.2901960784313726f32,
    g: 0.49019607843137253f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Subnautical: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.13333333333333333f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const SubterrainKingdom: Colour = Colour {
    r: 0.30980392156862746f32,
    g: 0.3058823529411765f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const SubtleBreeze: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.8235294117647058f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const Succubus: Colour = Colour {
    r: 0.6f32,
    g: 0.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Succulent: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.6431372549019608f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const SucculentLime: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.8666666666666667f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const Succulents: Colour = Colour {
    r: 0.0f32,
    g: 0.4666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const SuchAPeach: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.8666666666666667f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const SugarChic: Colour = Colour {
    r: 1.0f32,
    g: 0.8f32,
    b: 1.0f32,
    a: 1f32,
};
pub const SugarCoated: Colour = Colour {
    r: 1.0f32,
    g: 0.9294117647058824f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const SugarCookie: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8862745098039215f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const SugarGlaze: Colour = Colour {
    r: 1.0f32,
    g: 0.9411764705882353f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const SugarHigh: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.788235294117647f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const SugarMilk: Colour = Colour {
    r: 1.0f32,
    g: 0.9764705882352941f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const SugarMint: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.8862745098039215f32,
    b: 0.7725490196078432f32,
    a: 1f32,
};
pub const SugarQuill: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.8980392156862745f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const Sugarwinkle: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.7725490196078432f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const SulfurPit: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8f32,
    b: 0.4117647058823529f32,
    a: 1f32,
};
pub const Sulfuric: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9294117647058824f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const Sulphur: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.6901960784313725f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const SultanGold: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.6745098039215687f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const SultanOfPink: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.6078431372549019f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const SummerCrush: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.8392156862745098f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const SummerGlow: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const SummerMist: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.9176470588235294f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const SummerOf82: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.803921568627451f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const SummerSEnd: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.5764705882352941f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const Summit: Colour = Colour {
    r: 0.5450980392156862f32,
    g: 0.7137254901960784f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const SumptuousPurple: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.2980392156862745f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const SunFloodedWoods: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.8313725490196079f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const SunKissedCoral: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.403921568627451f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const SunkissedSands: Colour = Colour {
    r: 1.0f32,
    g: 0.9294117647058824f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const Sunbathed: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.8666666666666667f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const SunbathedBeach: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.8235294117647058f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const SunbathingBeauty: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.2784313725490196f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const Sunbeam: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9294117647058824f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const Sunburst: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.7098039215686275f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const Sunflower: Colour = Colour {
    r: 1.0f32,
    g: 0.7725490196078432f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const SunflowerIsland: Colour = Colour {
    r: 1.0f32,
    g: 0.803921568627451f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const SunflowerMango: Colour = Colour {
    r: 1.0f32,
    g: 0.7176470588235294f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SunflowerValley: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.7411764705882353f32,
    b: 0.15294117647058825f32,
    a: 1f32,
};
pub const SunkenGold: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.592156862745098f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SunkenHarbor: Colour = Colour {
    r: 0.10980392156862745f32,
    g: 0.23921568627450981f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const SunkissedBeach: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.6705882352941176f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const Sunlight: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.803921568627451f32,
    b: 0.5843137254901961f32,
    a: 1f32,
};
pub const SunnyGlory: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.8509803921568627f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const SunnyHoney: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9411764705882353f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const SunnyYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.9764705882352941f32,
    b: 0.09019607843137255f32,
    a: 1f32,
};
pub const Sunray: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.6705882352941176f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const SunsetGold: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.7647058823529411f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const SunsetOrange: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.3686274509803922f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const SunshineMellow: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.7607843137254902f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const SunshonePlum: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.4f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const SuperBanana: Colour = Colour {
    r: 1.0f32,
    g: 0.996078431372549f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const SuperPink: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.4196078431372549f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const SuperRareJade: Colour = Colour {
    r: 0.0784313725490196f32,
    g: 0.7294117647058823f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const SuperRoseRed: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.06274509803921569f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const SuperSaiyan: Colour = Colour {
    r: 1.0f32,
    g: 0.8666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const SuperSilver: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const SupremelyCool: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.7450980392156863f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const SurgicalGreen: Colour = Colour {
    r: 0.34901960784313724f32,
    g: 0.6431372549019608f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const SwampMonster: Colour = Colour {
    r: 0.0f32,
    g: 0.3333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const SwanDive: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8941176470588236f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const SwanLake: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.8980392156862745f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const SweetAndSassy: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.788235294117647f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const SweetButter: Colour = Colour {
    r: 1.0f32,
    g: 0.9882352941176471f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const SweetChilli: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.08627450980392157f32,
    b: 0.043137254901960784f32,
    a: 1f32,
};
pub const SweetCorn: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.8823529411764706f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const SweetDesire: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.2f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const SweetLilac: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.7098039215686275f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const SweetLucidDreams: Colour = Colour {
    r: 0.8f32,
    g: 0.7333333333333333f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const SweetMintPesto: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.9333333333333333f32,
    b: 0.6f32,
    a: 1f32,
};
pub const SweetPerfume: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.6039215686274509f32,
    b: 0.7254901960784313f32,
    a: 1f32,
};
pub const SweetPotato: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.48627450980392156f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const SweetVenom: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 1.0f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const Sweetly: Colour = Colour {
    r: 1.0f32,
    g: 0.8980392156862745f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const Swimmer: Colour = Colour {
    r: 0.0392156862745098f32,
    g: 0.5686274509803921f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Swimming: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.8980392156862745f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const Tabasco: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.15294117647058825f32,
    b: 0.07058823529411765f32,
    a: 1f32,
};
pub const Taco: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.7803921568627451f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const TadornaTeal: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.8431372549019608f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const TahiniBrown: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.5215686274509804f32,
    b: 0.4196078431372549f32,
    a: 1f32,
};
pub const Tamahagane: Colour = Colour {
    r: 0.23137254901960785f32,
    g: 0.24705882352941178f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const TamedBeast: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.14901960784313725f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const TamedBeauty: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.7372549019607844f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const Tandoori: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.3607843137254902f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const TangentPeriwinkle: Colour = Colour {
    r: 0.3137254901960784f32,
    g: 0.3137254901960784f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const TangerineTango: Colour = Colour {
    r: 1.0f32,
    g: 0.6196078431372549f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const TangledWeb: Colour = Colour {
    r: 0.6980392156862745f32,
    g: 0.6980392156862745f32,
    b: 0.6980392156862745f32,
    a: 1f32,
};
pub const TardisBlue: Colour = Colour {
    r: 0.0f32,
    g: 0.23137254901960785f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const TartanRed: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.1568627450980392f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const Tatami: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const Taupe: Colour = Colour {
    r: 0.7254901960784313f32,
    g: 0.6352941176470588f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const TeaGreen: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.9411764705882353f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const Teak: Colour = Colour {
    r: 0.6705882352941176f32,
    g: 0.5372549019607843f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const Teakwood: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.49411764705882355f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const Teal: Colour = Colour {
    r: 0.0f32,
    g: 0.5019607843137255f32,
    b: 0.5019607843137255f32,
    a: 1f32,
};
pub const TealMeNoLies: Colour = Colour {
    r: 0.050980392156862744f32,
    g: 0.6745098039215687f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const TealWithIt: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.4117647058823529f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Teardrop: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.9176470588235294f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const TechnoTaupe: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.7254901960784313f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const Technolust: Colour = Colour {
    r: 1.0f32,
    g: 0.5019607843137255f32,
    b: 0.9764705882352941f32,
    a: 1f32,
};
pub const Telemagenta: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.13333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Tempest: Colour = Colour {
    r: 0.4745098039215686f32,
    g: 0.5137254901960784f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const TemplarSGold: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9019607843137255f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const TemptatiousTangerine: Colour = Colour {
    r: 1.0f32,
    g: 0.4666666666666667f32,
    b: 0.2f32,
    a: 1f32,
};
pub const TenderShoot: Colour = Colour {
    r: 0.9098039215686274f32,
    g: 0.9176470588235294f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const TenderTaupe: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.6941176470588235f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const TennisBall: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 1.0f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const Terrestrial: Colour = Colour {
    r: 0.15294117647058825f32,
    g: 0.403921568627451f32,
    b: 0.3411764705882353f32,
    a: 1f32,
};
pub const Testosterose: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6666666666666666f32,
    b: 1.0f32,
    a: 1f32,
};
pub const ThaiChili: Colour = Colour {
    r: 0.807843137254902f32,
    g: 0.0f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const ThaiHot: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.10980392156862745f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const Thalassa: Colour = Colour {
    r: 0.3254901960784314f32,
    g: 0.6941176470588235f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const Thalassophile: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.6666666666666666f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const TheCountSBlack: Colour = Colour {
    r: 0.06274509803921569f32,
    g: 0.12549019607843137f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const TheDevilSGrass: Colour = Colour {
    r: 0.4f32,
    g: 0.39215686274509803f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const TheEnd: Colour = Colour {
    r: 0.16470588235294117f32,
    g: 0.16470588235294117f32,
    b: 0.16470588235294117f32,
    a: 1f32,
};
pub const TheGrapeWarOf97: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const TheLegendOfGreen: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const TheVastOfNight: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const ThinkPink: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.6470588235294118f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const ThorSThunder: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.8f32,
    b: 1.0f32,
    a: 1f32,
};
pub const ThreateningRed: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.011764705882352941f32,
    b: 0.0196078431372549f32,
    a: 1f32,
};
pub const ThrillingLime: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.7647058823529411f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const ThunderAndLightning: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9607843137254902f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const Thunderbird: Colour = Colour {
    r: 0.5725490196078431f32,
    g: 0.2196078431372549f32,
    b: 0.18823529411764706f32,
    a: 1f32,
};
pub const Thunderbolt: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9372549019607843f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const Thundercloud: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.5215686274509804f32,
    b: 0.5372549019607843f32,
    a: 1f32,
};
pub const ThymeAndPlace: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.5294117647058824f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const Tiger: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.611764705882353f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const TigerKing: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.6f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const TigerLily: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.34509803921568627f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const TigerOfMysore: Colour = Colour {
    r: 1.0f32,
    g: 0.5333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const TimelessBeauty: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.15294117647058825f32,
    b: 0.24313725490196078f32,
    a: 1f32,
};
pub const Tin: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.5686274509803921f32,
    b: 0.5686274509803921f32,
    a: 1f32,
};
pub const Toad: Colour = Colour {
    r: 0.4549019607843137f32,
    g: 0.5529411764705883f32,
    b: 0.4392156862745098f32,
    a: 1f32,
};
pub const ToadKing: Colour = Colour {
    r: 0.23921568627450981f32,
    g: 0.4235294117647059f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const Toadstool: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.1568627450980392f32,
    b: 0.1843137254901961f32,
    a: 1f32,
};
pub const ToastAndButter: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.6784313725490196f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const ToastedMarshmallowFluff: Colour = Colour {
    r: 1.0f32,
    g: 0.9764705882352941f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const ToastedPaprika: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.27450980392156865f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const Tobacco: Colour = Colour {
    r: 0.40784313725490196f32,
    g: 0.30980392156862746f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const TobaccoLeaf: Colour = Colour {
    r: 0.5490196078431373f32,
    g: 0.4470588235294118f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const ToesInTheSand: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8627450980392157f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Toffee: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.3176470588235294f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const Tofu: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8980392156862745f32,
    b: 0.8392156862745098f32,
    a: 1f32,
};
pub const Tomato: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.25098039215686274f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const TomatoBaby: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.050980392156862744f32,
    b: 0.09411764705882353f32,
    a: 1f32,
};
pub const TomatoBisque: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.34901960784313724f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const TomatoBurst: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.12549019607843137f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const TomatoQueen: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.26666666666666666f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Tonkatsu: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.6745098039215687f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const TooBigToWhale: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.5882352941176471f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const TooBlueToBeTrue: Colour = Colour {
    r: 0.0f32,
    g: 0.5333333333333333f32,
    b: 1.0f32,
    a: 1f32,
};
pub const TooDarkTonight: Colour = Colour {
    r: 0.0f32,
    g: 0.06666666666666667f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const TopiaryGreen: Colour = Colour {
    r: 0.4f32,
    g: 0.4666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Torchlight: Colour = Colour {
    r: 1.0f32,
    g: 0.788235294117647f32,
    b: 0.5215686274509804f32,
    a: 1f32,
};
pub const Toreador: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.07058823529411765f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const TorrefactoRoast: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.1411764705882353f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const Tortilla: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8588235294117647f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const Tostada: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.7568627450980392f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const TostyCrust: Colour = Colour {
    r: 0.6509803921568628f32,
    g: 0.49411764705882355f32,
    b: 0.29411764705882354f32,
    a: 1f32,
};
pub const TotalEclipse: Colour = Colour {
    r: 0.18823529411764706f32,
    g: 0.20784313725490197f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const TotallyBroccoli: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.596078431372549f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const TouchOfGlamor: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.5333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Toupe: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.6745098039215687f32,
    b: 0.49019607843137253f32,
    a: 1f32,
};
pub const ToxicBoyfriend: Colour = Colour {
    r: 0.8f32,
    g: 1.0f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const ToxicFrog: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.984313725490196f32,
    b: 0.596078431372549f32,
    a: 1f32,
};
pub const ToxicLatte: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.9725490196078431f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const ToxicSludge: Colour = Colour {
    r: 0.0f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const ToxicSteam: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.9921568627450981f32,
    b: 0.788235294117647f32,
    a: 1f32,
};
pub const TrackAndField: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.38823529411764707f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const TractorBeam: Colour = Colour {
    r: 0.0f32,
    g: 0.7490196078431373f32,
    b: 0.996078431372549f32,
    a: 1f32,
};
pub const Tradewind: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.7725490196078432f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const TrafficGreen: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 1.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const TrafficRed: Colour = Colour {
    r: 1.0f32,
    g: 0.10980392156862745f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const TrafficYellow: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.8627450980392157f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const TrailDust: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.7686274509803922f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const TranquiliTeal: Colour = Colour {
    r: 0.4235294117647059f32,
    g: 0.615686274509804f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const Transcendence: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.9568627450980393f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const Transfusion: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.09411764705882353f32,
    b: 0.2f32,
    a: 1f32,
};
pub const TranslucentUnicorn: Colour = Colour {
    r: 1.0f32,
    g: 0.9294117647058824f32,
    b: 0.9372549019607843f32,
    a: 1f32,
};
pub const TrappedDarkness: Colour = Colour {
    r: 0.054901960784313725f32,
    g: 0.11372549019607843f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const Treasure: Colour = Colour {
    r: 0.9058823529411765f32,
    g: 0.8156862745098039f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const TreasureChest: Colour = Colour {
    r: 0.4470588235294118f32,
    g: 0.40784313725490196f32,
    b: 0.32941176470588235f32,
    a: 1f32,
};
pub const TreasureMap: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.7333333333333333f32,
    b: 0.615686274509804f32,
    a: 1f32,
};
pub const TreasureMapWaters: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.5607843137254902f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const TreasuredTeal: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.7568627450980392f32,
    b: 0.7019607843137254f32,
    a: 1f32,
};
pub const Treasures: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.5450980392156862f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const Treasury: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.8196078431372549f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const TreeHugger: Colour = Colour {
    r: 0.4745098039215686f32,
    g: 0.4666666666666667f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Treetop: Colour = Colour {
    r: 0.5686274509803921f32,
    g: 0.7137254901960784f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const TreetopCathedral: Colour = Colour {
    r: 0.1843137254901961f32,
    g: 0.2901960784313726f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const TrippyVelvet: Colour = Colour {
    r: 0.8f32,
    g: 0.0f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const TropicalDream: Colour = Colour {
    r: 0.8509803921568627f32,
    g: 0.9176470588235294f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const TropicalEscape: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.7333333333333333f32,
    b: 0.6862745098039216f32,
    a: 1f32,
};
pub const TropicalFog: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.792156862745098f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const TropicalForest: Colour = Colour {
    r: 0.00784313725490196f32,
    g: 0.2901960784313726f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const TropicalFreeze: Colour = Colour {
    r: 0.6f32,
    g: 0.8666666666666667f32,
    b: 0.8f32,
    a: 1f32,
};
pub const TropicalFunk: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const TropicalMist: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.9098039215686274f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const TropicalRain: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.4666666666666667f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const TropicalRainforest: Colour = Colour {
    r: 0.0f32,
    g: 0.4588235294117647f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const TropicalTurquoise: Colour = Colour {
    r: 0.01568627450980392f32,
    g: 0.803921568627451f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Trout: Colour = Colour {
    r: 0.2980392156862745f32,
    g: 0.3254901960784314f32,
    b: 0.33725490196078434f32,
    a: 1f32,
};
pub const TroutCaviar: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.3254901960784314f32,
    b: 0.0f32,
    a: 1f32,
};
pub const TrueBlue: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.058823529411764705f32,
    b: 0.8f32,
    a: 1f32,
};
pub const TruffleTrouble: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.3176470588235294f32,
    b: 0.2235294117647059f32,
    a: 1f32,
};
pub const TrumpetGold: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.7058823529411765f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const Tulip: Colour = Colour {
    r: 1.0f32,
    g: 0.5294117647058824f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const Tuna: Colour = Colour {
    r: 0.27450980392156865f32,
    g: 0.28627450980392155f32,
    b: 0.3058823529411765f32,
    a: 1f32,
};
pub const TunicGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.8f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Turf: Colour = Colour {
    r: 0.2549019607843137f32,
    g: 0.3568627450980392f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const TurfMaster: Colour = Colour {
    r: 0.0f32,
    g: 0.6f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const TurkishJade: Colour = Colour {
    r: 0.16862745098039217f32,
    g: 0.5333333333333333f32,
    b: 0.5529411764705883f32,
    a: 1f32,
};
pub const Turquoise: Colour = Colour {
    r: 0.023529411764705882f32,
    g: 0.7607843137254902f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const TurquoiseFantasies: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.6862745098039216f32,
    b: 0.6549019607843137f32,
    a: 1f32,
};
pub const TurquoisePearl: Colour = Colour {
    r: 0.5372549019607843f32,
    g: 0.9607843137254902f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const TurquoiseTortoise: Colour = Colour {
    r: 0.27058823529411763f32,
    g: 0.4823529411764706f32,
    b: 0.4549019607843137f32,
    a: 1f32,
};
pub const Turtle: Colour = Colour {
    r: 0.3215686274509804f32,
    g: 0.24705882352941178f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const TurtleWarrior: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.7176470588235294f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const Tuscan: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.8352941176470589f32,
    b: 0.6509803921568628f32,
    a: 1f32,
};
pub const TuscanSun: Colour = Colour {
    r: 1.0f32,
    g: 0.8470588235294118f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const Tussiemussie: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.7725490196078432f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const Tutu: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8941176470588236f32,
    b: 0.8901960784313725f32,
    a: 1f32,
};
pub const Tuxedo: Colour = Colour {
    r: 0.24705882352941178f32,
    g: 0.23529411764705882f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const Twilight: Colour = Colour {
    r: 0.3058823529411765f32,
    g: 0.3176470588235294f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const TwilightMeadow: Colour = Colour {
    r: 0.3176470588235294f32,
    g: 0.6470588235294118f32,
    b: 0.6431372549019608f32,
    a: 1f32,
};
pub const TwilightZone: Colour = Colour {
    r: 0.09803921568627451f32,
    g: 0.09803921568627451f32,
    b: 0.08627450980392157f32,
    a: 1f32,
};
pub const TwinkleNight: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.4235294117647059f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const TwinklePink: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.8470588235294118f32,
    b: 0.8f32,
    a: 1f32,
};
pub const TwinklyPinkily: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.2784313725490196f32,
    b: 0.5882352941176471f32,
    a: 1f32,
};
pub const UltimateGrey: Colour = Colour {
    r: 0.6627450980392157f32,
    g: 0.6588235294117647f32,
    b: 0.6627450980392157f32,
    a: 1f32,
};
pub const UltraGreen: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.7294117647058823f32,
    b: 0.30196078431372547f32,
    a: 1f32,
};
pub const UltraMint: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.9372549019607843f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const UltraMoss: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.9529411764705882f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Ultraberry: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.0f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const Umbra: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.11764705882352941f32,
    b: 0.12156862745098039f32,
    a: 1f32,
};
pub const UnderTheSea: Colour = Colour {
    r: 0.2235294117647059f32,
    g: 0.36470588235294116f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const Underground: Colour = Colour {
    r: 0.4f32,
    g: 0.35294117647058826f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const UnderpassShrine: Colour = Colour {
    r: 0.8f32,
    g: 0.26666666666666666f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const UnderwaterMoonlight: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.5333333333333333f32,
    b: 0.6666666666666666f32,
    a: 1f32,
};
pub const UnderwaterWorld: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.4980392156862745f32,
    b: 0.47843137254901963f32,
    a: 1f32,
};
pub const Underworld: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.13725490196078433f32,
    b: 0.10980392156862745f32,
    a: 1f32,
};
pub const UnicornDust: Colour = Colour {
    r: 1.0f32,
    g: 0.1843137254901961f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const UrbanChic: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.2980392156862745f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const UrbanSnowfall: Colour = Colour {
    r: 0.8588235294117647f32,
    g: 0.8470588235294118f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const VaVaVoom: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.7019607843137254f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const ValentineLava: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.027450980392156862f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const ValentineSKiss: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.2f32,
    b: 0.39215686274509803f32,
    a: 1f32,
};
pub const Valkyrie: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const ValleyOfTears: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.8823529411764706f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const VampireBite: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.00784313725490196f32,
    b: 0.2f32,
    a: 1f32,
};
pub const VampireFangs: Colour = Colour {
    r: 0.8f32,
    g: 0.13333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const VampireFiction: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.058823529411764705f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const VampireHunter: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.0196078431372549f32,
    b: 0.027450980392156862f32,
    a: 1f32,
};
pub const VampireLoveStory: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.0f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const VampireRed: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.2549019607843137f32,
    b: 0.19607843137254902f32,
    a: 1f32,
};
pub const VampireStateBuilding: Colour = Colour {
    r: 0.8f32,
    g: 0.06666666666666667f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Vampirella: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.1568627450980392f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const VampiricBloodlust: Colour = Colour {
    r: 0.8f32,
    g: 0.0f32,
    b: 0.4f32,
    a: 1f32,
};
pub const VanDykeBrown: Colour = Colour {
    r: 0.4f32,
    g: 0.25882352941176473f32,
    b: 0.1568627450980392f32,
    a: 1f32,
};
pub const Vanilla: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.8980392156862745f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const VanillaBlush: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9294117647058824f32,
    b: 0.8941176470588236f32,
    a: 1f32,
};
pub const VanillaCake: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.9411764705882353f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const VanillaCream: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8901960784313725f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const VanillaDrop: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.9215686274509803f32,
    a: 1f32,
};
pub const VanillaIce: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9490196078431372f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const VanillaSugar: Colour = Colour {
    r: 0.9450980392156862f32,
    g: 0.9098039215686274f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const VanishingPoint: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.9333333333333333f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Vapor: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 1.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const VaporTrail: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9333333333333333f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Vaporwave: Colour = Colour {
    r: 1.0f32,
    g: 0.4f32,
    b: 0.9333333333333333f32,
    a: 1f32,
};
pub const VaporwaveBlue: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.8666666666666667f32,
    b: 1.0f32,
    a: 1f32,
};
pub const VarnishedIvory: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.8627450980392157f32,
    b: 0.8f32,
    a: 1f32,
};
pub const VegasGold: Colour = Colour {
    r: 0.7725490196078432f32,
    g: 0.7019607843137254f32,
    b: 0.34509803921568627f32,
    a: 1f32,
};
pub const Vegetarian: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Vegetation: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.803921568627451f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const VeiledRose: Colour = Colour {
    r: 0.9686274509803922f32,
    g: 0.803921568627451f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const VeiledTreasure: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9294117647058824f32,
    b: 0.7137254901960784f32,
    a: 1f32,
};
pub const VeilingWaterfalls: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.9176470588235294f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Velvet: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.03137254901960784f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const VelvetBlack: Colour = Colour {
    r: 0.1411764705882353f32,
    g: 0.12156862745098039f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const VelvetCosmos: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const VelvetMagic: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.06666666666666667f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const VelvetScarf: Colour = Colour {
    r: 0.8901960784313725f32,
    g: 0.8745098039215686f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const VelvetWine: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.2627450980392157f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const VenomDart: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 1.0f32,
    b: 0.00392156862745098f32,
    a: 1f32,
};
pub const VenomousGreen: Colour = Colour {
    r: 0.4f32,
    g: 1.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Venus: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8156862745098039f32,
    b: 0.3254901960784314f32,
    a: 1f32,
};
pub const VenusMist: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.3764705882352941f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const VenusSlipperOrchid: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.45098039215686275f32,
    b: 1.0f32,
    a: 1f32,
};
pub const VenusViolet: Colour = Colour {
    r: 0.47843137254901963f32,
    g: 0.42745098039215684f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const VerandaGold: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.6f32,
    b: 0.40784313725490196f32,
    a: 1f32,
};
pub const VerdantForest: Colour = Colour {
    r: 0.1568627450980392f32,
    g: 0.3803921568627451f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const VerdantLeaf: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.48627450980392156f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Verde: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.7019607843137254f32,
    b: 0.5137254901960784f32,
    a: 1f32,
};
pub const Verdigris: Colour = Colour {
    r: 0.2627450980392157f32,
    g: 0.7019607843137254f32,
    b: 0.6823529411764706f32,
    a: 1f32,
};
pub const Vermicelles: Colour = Colour {
    r: 0.8549019607843137f32,
    g: 0.7450980392156863f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const Vermilion: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.19607843137254902f32,
    b: 0.047058823529411764f32,
    a: 1f32,
};
pub const VermilionScarlet: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.023529411764705882f32,
    b: 0.16862745098039217f32,
    a: 1f32,
};
pub const Verminal: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.8f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const VertigoCherry: Colour = Colour {
    r: 0.6f32,
    g: 0.0f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const VeryBerry: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.2f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const VeryCoffee: Colour = Colour {
    r: 0.4f32,
    g: 0.26666666666666666f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const VibrantAmber: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.5647058823529412f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const VibrantBlue: Colour = Colour {
    r: 0.011764705882352941f32,
    g: 0.2235294117647059f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const VibrantHoney: Colour = Colour {
    r: 1.0f32,
    g: 0.7411764705882353f32,
    b: 0.19215686274509805f32,
    a: 1f32,
};
pub const VibrantMint: Colour = Colour {
    r: 0.0f32,
    g: 1.0f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const VibrantOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.3843137254901961f32,
    b: 0.08627450980392157f32,
    a: 1f32,
};
pub const VibrantPurple: Colour = Colour {
    r: 0.6784313725490196f32,
    g: 0.011764705882352941f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const VibrantVelvet: Colour = Colour {
    r: 0.7333333333333333f32,
    g: 0.0f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const VibrantVine: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.21568627450980393f32,
    b: 0.22745098039215686f32,
    a: 1f32,
};
pub const VibrantYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.8549019607843137f32,
    b: 0.1607843137254902f32,
    a: 1f32,
};
pub const ViceCity: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.0f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const ViciousViolet: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.3137254901960784f32,
    b: 0.615686274509804f32,
    a: 1f32,
};
pub const VictorianCrown: Colour = Colour {
    r: 0.7647058823529411f32,
    g: 0.5450980392156862f32,
    b: 0.21176470588235294f32,
    a: 1f32,
};
pub const VictorianGarden: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.5568627450980392f32,
    b: 0.2980392156862745f32,
    a: 1f32,
};
pub const ViennaRoast: Colour = Colour {
    r: 0.2f32,
    g: 0.0f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Viking: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.6941176470588235f32,
    b: 0.7843137254901961f32,
    a: 1f32,
};
pub const VikingDiva: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7294117647058823f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const VinCuit: Colour = Colour {
    r: 0.7058823529411765f32,
    g: 0.4549019607843137f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const Vinaceous: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.6f32,
    b: 0.5803921568627451f32,
    a: 1f32,
};
pub const VinaceousCinnamon: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.5450980392156862f32,
    b: 0.5450980392156862f32,
    a: 1f32,
};
pub const VinaceousTawny: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.2627450980392157f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Vinaigrette: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8549019607843137f32,
    b: 0.6823529411764706f32,
    a: 1f32,
};
pub const Vineyard: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.6196078431372549f32,
    b: 0.5176470588235295f32,
    a: 1f32,
};
pub const Vintage: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.4588235294117647f32,
    b: 0.5725490196078431f32,
    a: 1f32,
};
pub const VintageCopper: Colour = Colour {
    r: 0.615686274509804f32,
    g: 0.37254901960784315f32,
    b: 0.27450980392156865f32,
    a: 1f32,
};
pub const VintagePorcelain: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9294117647058824f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const Viola: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.43137254901960786f32,
    b: 0.7411764705882353f32,
    a: 1f32,
};
pub const Violaceous: Colour = Colour {
    r: 0.7490196078431373f32,
    g: 0.5607843137254902f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const ViolentViolet: Colour = Colour {
    r: 0.4980392156862745f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Violet: Colour = Colour {
    r: 0.6039215686274509f32,
    g: 0.054901960784313725f32,
    b: 0.9176470588235294f32,
    a: 1f32,
};
pub const VioletHeaven: Colour = Colour {
    r: 0.803921568627451f32,
    g: 0.7176470588235294f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const VioletKiss: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.6274509803921569f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const VioletPink: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.37254901960784315f32,
    b: 0.9882352941176471f32,
    a: 1f32,
};
pub const VioletPoison: Colour = Colour {
    r: 0.5254901960784314f32,
    g: 0.00392156862745098f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const VioletVapor: Colour = Colour {
    r: 0.8980392156862745f32,
    g: 0.8549019607843137f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const VioletVelvet: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.611764705882353f32,
    b: 0.8509803921568627f32,
    a: 1f32,
};
pub const VioletVision: Colour = Colour {
    r: 0.7176470588235294f32,
    g: 0.7411764705882353f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const VioletVixen: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.2f32,
    b: 0.4666666666666667f32,
    a: 1f32,
};
pub const VioletVogue: Colour = Colour {
    r: 0.9137254901960784f32,
    g: 0.8823529411764706f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const VirginOliveOil: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.8627450980392157f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const Viridian: Colour = Colour {
    r: 0.11764705882352941f32,
    g: 0.5686274509803921f32,
    b: 0.403921568627451f32,
    a: 1f32,
};
pub const VirtualBoy: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.00784313725490196f32,
    b: 0.08235294117647059f32,
    a: 1f32,
};
pub const VirtualGolf: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.9333333333333333f32,
    b: 0.07450980392156863f32,
    a: 1f32,
};
pub const Vitality: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.6078431372549019f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const VitaminC: Colour = Colour {
    r: 1.0f32,
    g: 0.6f32,
    b: 0.0f32,
    a: 1f32,
};
pub const VividBlue: Colour = Colour {
    r: 0.08235294117647059f32,
    g: 0.1803921568627451f32,
    b: 1.0f32,
    a: 1f32,
};
pub const VividOrange: Colour = Colour {
    r: 1.0f32,
    g: 0.37254901960784315f32,
    b: 0.0f32,
    a: 1f32,
};
pub const VividRaspberry: Colour = Colour {
    r: 1.0f32,
    g: 0.0f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const VividViolet: Colour = Colour {
    r: 0.6235294117647059f32,
    g: 0.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Void: Colour = Colour {
    r: 0.0196078431372549f32,
    g: 0.050980392156862744f32,
    b: 0.1450980392156863f32,
    a: 1f32,
};
pub const Voila: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.5450980392156862f32,
    b: 0.6588235294117647f32,
    a: 1f32,
};
pub const VolcanicAsh: Colour = Colour {
    r: 0.43529411764705883f32,
    g: 0.4627450980392157f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const VolcanicIsland: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.3215686274509804f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const VolcanicRock: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.4117647058823529f32,
    b: 0.396078431372549f32,
    a: 1f32,
};
pub const Voldemort: Colour = Colour {
    r: 0.17647058823529413f32,
    g: 0.07450980392156863f32,
    b: 0.37254901960784315f32,
    a: 1f32,
};
pub const Volt: Colour = Colour {
    r: 0.807843137254902f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const VoluptuousViolet: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.06666666666666667f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const Voodoo: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.19607843137254902f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const VoraciousWhite: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.9333333333333333f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const Vulcan: Colour = Colour {
    r: 0.21176470588235294f32,
    g: 0.2196078431372549f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const VulcanFire: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.2235294117647059f32,
    b: 0.050980392156862744f32,
    a: 1f32,
};
pub const Vulcanized: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.26666666666666666f32,
    b: 0.2627450980392157f32,
    a: 1f32,
};
pub const WaffleCone: Colour = Colour {
    r: 0.8862745098039215f32,
    g: 0.7803921568627451f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const Waikiki: Colour = Colour {
    r: 0.12941176470588237f32,
    g: 0.5450980392156862f32,
    b: 0.6274509803921569f32,
    a: 1f32,
};
pub const WakameGreen: Colour = Colour {
    r: 0.0f32,
    g: 0.396078431372549f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const WalkInThePark: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.7333333333333333f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const WalkieChalkie: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9607843137254902f32,
    b: 0.9803921568627451f32,
    a: 1f32,
};
pub const Walkway: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.6f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const WalledGarden: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.8f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Walnut: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.24705882352941178f32,
    b: 0.10196078431372549f32,
    a: 1f32,
};
pub const WalnutMilkies: Colour = Colour {
    r: 1.0f32,
    g: 0.9411764705882353f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const Walrus: Colour = Colour {
    r: 0.6f32,
    g: 0.6078431372549019f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const WarPaintRed: Colour = Colour {
    r: 0.8627450980392157f32,
    g: 0.3411764705882353f32,
    b: 0.11372549019607843f32,
    a: 1f32,
};
pub const Warlord: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.0f32,
    b: 0.2f32,
    a: 1f32,
};
pub const WarmAsh: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.788235294117647f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const WarmBlue: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.3411764705882353f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const WarmBrown: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.3058823529411765f32,
    b: 0.00784313725490196f32,
    a: 1f32,
};
pub const WarmLight: Colour = Colour {
    r: 1.0f32,
    g: 0.9764705882352941f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const WarmNeutral: Colour = Colour {
    r: 0.7568627450980392f32,
    g: 0.6941176470588235f32,
    b: 0.615686274509804f32,
    a: 1f32,
};
pub const WarmOats: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.8117647058823529f32,
    b: 0.7294117647058823f32,
    a: 1f32,
};
pub const WarmPink: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.3333333333333333f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const WarmPurple: Colour = Colour {
    r: 0.5843137254901961f32,
    g: 0.1803921568627451f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const WarmSpring: Colour = Colour {
    r: 0.25882352941176473f32,
    g: 0.5254901960784314f32,
    b: 0.7372549019607844f32,
    a: 1f32,
};
pub const WarmWelcome: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.5647058823529412f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const WarmWhite: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9215686274509803f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const WarmingHeart: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.29411764705882354f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const WarpDrive: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.9490196078431372f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const Warrior: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.40784313725490196f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const WarriorQueen: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.17647058823529413f32,
    b: 0.2823529411764706f32,
    a: 1f32,
};
pub const Wasabi: Colour = Colour {
    r: 0.6862745098039216f32,
    g: 0.8431372549019608f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const WasabiNori: Colour = Colour {
    r: 0.2f32,
    g: 0.2f32,
    b: 0.0f32,
    a: 1f32,
};
pub const WashedCanvas: Colour = Colour {
    r: 0.9529411764705882f32,
    g: 0.9411764705882353f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const WashedDollar: Colour = Colour {
    r: 0.8823529411764706f32,
    g: 0.8901960784313725f32,
    b: 0.8431372549019608f32,
    a: 1f32,
};
pub const Wasteland: Colour = Colour {
    r: 0.611764705882353f32,
    g: 0.5333333333333333f32,
    b: 0.3333333333333333f32,
    a: 1f32,
};
pub const Water: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.9450980392156862f32,
    b: 0.9764705882352941f32,
    a: 1f32,
};
pub const WaterLeaf: Colour = Colour {
    r: 0.7137254901960784f32,
    g: 0.9254901960784314f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const WaterLily: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8901960784313725f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const WaterNymph: Colour = Colour {
    r: 0.5058823529411764f32,
    g: 0.8156862745098039f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const WaterPark: Colour = Colour {
    r: 0.32941176470588235f32,
    g: 0.6862745098039216f32,
    b: 0.611764705882353f32,
    a: 1f32,
};
pub const Waterfall: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.6901960784313725f32,
    b: 0.6352941176470588f32,
    a: 1f32,
};
pub const Watermelon: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.27450980392156865f32,
    b: 0.34901960784313724f32,
    a: 1f32,
};
pub const WatermelonGelato: Colour = Colour {
    r: 0.7529411764705882f32,
    g: 0.40784313725490196f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const WatermelonMilk: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8117647058823529f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const WatermelonMousse: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.8784313725490196f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const WatermelonSugar: Colour = Colour {
    r: 0.8941176470588236f32,
    g: 0.16862745098039217f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const Watermelonade: Colour = Colour {
    r: 0.9215686274509803f32,
    g: 0.27450980392156865f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const Waterworld: Colour = Colour {
    r: 0.0f32,
    g: 0.44313725490196076f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const Wave: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.807843137254902f32,
    b: 0.8352941176470589f32,
    a: 1f32,
};
pub const WaveSplash: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.8941176470588236f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Wavelet: Colour = Colour {
    r: 0.49019607843137253f32,
    g: 0.7686274509803922f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const WavyNavy: Colour = Colour {
    r: 0.0f32,
    g: 0.396078431372549f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const Wax: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.7333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const WaxFlower: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.7019607843137254f32,
    b: 0.6196078431372549f32,
    a: 1f32,
};
pub const WaxyCorn: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.7098039215686275f32,
    b: 0.0f32,
    a: 1f32,
};
pub const WayBeyondTheBlue: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.5333333333333333f32,
    b: 0.8f32,
    a: 1f32,
};
pub const WePeep: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8431372549019608f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const WeatheredLeather: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.3803921568627451f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const WeatheredStone: Colour = Colour {
    r: 0.7686274509803922f32,
    g: 0.7725490196078432f32,
    b: 0.7764705882352941f32,
    a: 1f32,
};
pub const WeatheredWood: Colour = Colour {
    r: 0.6941176470588235f32,
    g: 0.611764705882353f32,
    b: 0.5254901960784314f32,
    a: 1f32,
};
pub const WeddingDress: Colour = Colour {
    r: 0.996078431372549f32,
    g: 0.996078431372549f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const WeddingInWhite: Colour = Colour {
    r: 1.0f32,
    g: 0.996078431372549f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const Weissbier: Colour = Colour {
    r: 0.7019607843137254f32,
    g: 0.5137254901960784f32,
    b: 0.23137254901960785f32,
    a: 1f32,
};
pub const WetAsphalt: Colour = Colour {
    r: 0.596078431372549f32,
    g: 0.611764705882353f32,
    b: 0.6705882352941176f32,
    a: 1f32,
};
pub const WetConcrete: Colour = Colour {
    r: 0.20784313725490197f32,
    g: 0.2196078431372549f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const WetTaupe: Colour = Colour {
    r: 0.5647058823529412f32,
    g: 0.49411764705882355f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Whale: Colour = Colour {
    r: 0.48627450980392156f32,
    g: 0.5058823529411764f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const WhaleShark: Colour = Colour {
    r: 0.3764705882352941f32,
    g: 0.48627450980392156f32,
    b: 0.5568627450980392f32,
    a: 1f32,
};
pub const WhaleSTale: Colour = Colour {
    r: 0.06666666666666667f32,
    g: 0.35294117647058826f32,
    b: 0.5098039215686274f32,
    a: 1f32,
};
pub const WhatWeDoInTheShadows: Colour = Colour {
    r: 0.26666666666666666f32,
    g: 0.06666666666666667f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const Wheat: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.8666666666666667f32,
    b: 0.49411764705882355f32,
    a: 1f32,
};
pub const WheatSheaf: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8313725490196079f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const WhereThereIsSmoke: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.8f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const WhippedCream: Colour = Colour {
    r: 0.9490196078431372f32,
    g: 0.9411764705882353f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const Whirlpool: Colour = Colour {
    r: 0.6470588235294118f32,
    g: 0.8470588235294118f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Whiskers: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9450980392156862f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const Whiskey: Colour = Colour {
    r: 0.8235294117647058f32,
    g: 0.5647058823529412f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const WhiskeyAndWine: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.27450980392156865f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const WhiskeySour: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.5686274509803921f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const Whisky: Colour = Colour {
    r: 0.7607843137254902f32,
    g: 0.5294117647058824f32,
    b: 0.4823529411764706f32,
    a: 1f32,
};
pub const WhiskyBarrel: Colour = Colour {
    r: 0.5882352941176471f32,
    g: 0.4549019607843137f32,
    b: 0.3568627450980392f32,
    a: 1f32,
};
pub const WhiskyCola: Colour = Colour {
    r: 0.4666666666666667f32,
    g: 0.13333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const WhiskySour: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.2f32,
    a: 1f32,
};
pub const WhisperOfSmoke: Colour = Colour {
    r: 0.796078431372549f32,
    g: 0.807843137254902f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const WhisperOfWhite: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8588235294117647f32,
    b: 0.792156862745098f32,
    a: 1f32,
};
pub const WhisperWhite: Colour = Colour {
    r: 0.9176470588235294f32,
    g: 0.8862745098039215f32,
    b: 0.8274509803921568f32,
    a: 1f32,
};
pub const WhisperingSmoke: Colour = Colour {
    r: 0.8470588235294118f32,
    g: 0.8470588235294118f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const WhisperyBreeze: Colour = Colour {
    r: 0.8392156862745098f32,
    g: 0.9137254901960784f32,
    b: 0.9019607843137255f32,
    a: 1f32,
};
pub const White: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 1.0f32,
    a: 1f32,
};
pub const WhiteAsparagus: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.9176470588235294f32,
    b: 0.7450980392156863f32,
    a: 1f32,
};
pub const WhiteBeach: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9372549019607843f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const WhiteBullet: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8745098039215686f32,
    b: 0.8549019607843137f32,
    a: 1f32,
};
pub const WhiteChalk: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.9568627450980393f32,
    b: 0.9450980392156862f32,
    a: 1f32,
};
pub const WhiteChocolate: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8901960784313725f32,
    b: 0.7803921568627451f32,
    a: 1f32,
};
pub const WhiteChristmas: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.9098039215686274f32,
    b: 0.9098039215686274f32,
    a: 1f32,
};
pub const WhiteElephant: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.8705882352941177f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const WhiteFrost: Colour = Colour {
    r: 0.8705882352941177f32,
    g: 0.9019607843137255f32,
    b: 0.9254901960784314f32,
    a: 1f32,
};
pub const WhiteGlove: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.9372549019607843f32,
    b: 0.9294117647058824f32,
    a: 1f32,
};
pub const WhiteMecca: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.9529411764705882f32,
    b: 0.8823529411764706f32,
    a: 1f32,
};
pub const WhitePearl: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.8823529411764706f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const WhitePorcelain: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.984313725490196f32,
    b: 0.9725490196078431f32,
    a: 1f32,
};
pub const WhiteRussian: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8784313725490196f32,
    b: 0.8627450980392157f32,
    a: 1f32,
};
pub const WhiteSand: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9215686274509803f32,
    b: 0.8470588235294118f32,
    a: 1f32,
};
pub const WhiteSmoke: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9607843137254902f32,
    b: 0.9607843137254902f32,
    a: 1f32,
};
pub const WhiteStrawberry: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.8901960784313725f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const WhiteTruffle: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.8588235294117647f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const WhiteWarmWool: Colour = Colour {
    r: 0.9372549019607843f32,
    g: 0.9019607843137255f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const WickedGreen: Colour = Colour {
    r: 0.6078431372549019f32,
    g: 0.792156862745098f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const WickedWitch: Colour = Colour {
    r: 0.3568627450980392f32,
    g: 0.596078431372549f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const Widowmaker: Colour = Colour {
    r: 0.6f32,
    g: 0.6666666666666666f32,
    b: 1.0f32,
    a: 1f32,
};
pub const WildBerry: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.22745098039215686f32,
    b: 0.23529411764705882f32,
    a: 1f32,
};
pub const WildChocolate: Colour = Colour {
    r: 0.4f32,
    g: 0.3176470588235294f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const WildForest: Colour = Colour {
    r: 0.2196078431372549f32,
    g: 0.5686274509803921f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const WildHorses: Colour = Colour {
    r: 0.5529411764705883f32,
    g: 0.403921568627451f32,
    b: 0.2784313725490196f32,
    a: 1f32,
};
pub const WildRice: Colour = Colour {
    r: 0.8352941176470589f32,
    g: 0.7490196078431373f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const WildViolet: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.12549019607843137f32,
    b: 0.6078431372549019f32,
    a: 1f32,
};
pub const WildWest: Colour = Colour {
    r: 0.49411764705882355f32,
    g: 0.3607843137254902f32,
    b: 0.3215686274509804f32,
    a: 1f32,
};
pub const WildWheat: Colour = Colour {
    r: 0.8784313725490196f32,
    g: 0.8823529411764706f32,
    b: 0.8196078431372549f32,
    a: 1f32,
};
pub const Wilderness: Colour = Colour {
    r: 0.5607843137254902f32,
    g: 0.5333333333333333f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const Wildfire: Colour = Colour {
    r: 1.0f32,
    g: 0.5333333333333333f32,
    b: 0.2f32,
    a: 1f32,
};
pub const WillowLeaf: Colour = Colour {
    r: 0.6313725490196078f32,
    g: 0.6431372549019608f32,
    b: 0.42745098039215684f32,
    a: 1f32,
};
pub const WindBlown: Colour = Colour {
    r: 0.8666666666666667f32,
    g: 0.8901960784313725f32,
    b: 0.9058823529411765f32,
    a: 1f32,
};
pub const WindChime: Colour = Colour {
    r: 0.8745098039215686f32,
    g: 0.8784313725490196f32,
    b: 0.8862745098039215f32,
    a: 1f32,
};
pub const WindChimes: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.7725490196078432f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const Windfall: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.6549019607843137f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const Windjammer: Colour = Colour {
    r: 0.3843137254901961f32,
    g: 0.6470588235294118f32,
    b: 0.8745098039215686f32,
    a: 1f32,
};
pub const Windows95Desktop: Colour = Colour {
    r: 0.00392156862745098f32,
    g: 0.5098039215686274f32,
    b: 0.5058823529411764f32,
    a: 1f32,
};
pub const WindsorToffee: Colour = Colour {
    r: 0.8f32,
    g: 0.7058823529411765f32,
    b: 0.5647058823529412f32,
    a: 1f32,
};
pub const Windstorm: Colour = Colour {
    r: 0.42745098039215684f32,
    g: 0.596078431372549f32,
    b: 0.7686274509803922f32,
    a: 1f32,
};
pub const Windsurfing: Colour = Colour {
    r: 0.22745098039215686f32,
    g: 0.4392156862745098f32,
    b: 0.6f32,
    a: 1f32,
};
pub const Windy: Colour = Colour {
    r: 0.7411764705882353f32,
    g: 0.8196078431372549f32,
    b: 0.8235294117647058f32,
    a: 1f32,
};
pub const WindyMeadow: Colour = Colour {
    r: 0.6901960784313725f32,
    g: 0.6509803921568628f32,
    b: 0.4627450980392157f32,
    a: 1f32,
};
pub const WineAndRoses: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.20784313725490197f32,
    b: 0.25098039215686274f32,
    a: 1f32,
};
pub const WineBarrel: Colour = Colour {
    r: 0.6666666666666666f32,
    g: 0.3333333333333333f32,
    b: 0.13333333333333333f32,
    a: 1f32,
};
pub const WineCellar: Colour = Colour {
    r: 0.4392156862745098f32,
    g: 0.25098039215686274f32,
    b: 0.23921568627450981f32,
    a: 1f32,
};
pub const WineGrape: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.09019607843137255f32,
    b: 0.3176470588235294f32,
    a: 1f32,
};
pub const WineStain: Colour = Colour {
    r: 0.4117647058823529f32,
    g: 0.26666666666666666f32,
    b: 0.30980392156862746f32,
    a: 1f32,
};
pub const WineTasting: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.16470588235294117f32,
    b: 0.20392156862745098f32,
    a: 1f32,
};
pub const WineTour: Colour = Colour {
    r: 0.396078431372549f32,
    g: 0.23137254901960785f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Wineberry: Colour = Colour {
    r: 0.4f32,
    g: 0.2f32,
    b: 0.4f32,
    a: 1f32,
};
pub const WingCommander: Colour = Colour {
    r: 0.0f32,
    g: 0.396078431372549f32,
    b: 0.6745098039215687f32,
    a: 1f32,
};
pub const WinterDuvet: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const WinterLakes: Colour = Colour {
    r: 0.3607843137254902f32,
    g: 0.592156862745098f32,
    b: 0.8117647058823529f32,
    a: 1f32,
};
pub const WinterScene: Colour = Colour {
    r: 0.7450980392156863f32,
    g: 0.807843137254902f32,
    b: 0.8588235294117647f32,
    a: 1f32,
};
pub const WinterStorm: Colour = Colour {
    r: 0.29411764705882354f32,
    g: 0.4392156862745098f32,
    b: 0.4745098039215686f32,
    a: 1f32,
};
pub const WinterWizard: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.9019607843137255f32,
    b: 1.0f32,
    a: 1f32,
};
pub const Wintermint: Colour = Colour {
    r: 0.5803921568627451f32,
    g: 0.8235294117647058f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const WishingWell: Colour = Colour {
    r: 0.8156862745098039f32,
    g: 0.8196078431372549f32,
    b: 0.7568627450980392f32,
    a: 1f32,
};
pub const Wisteria: Colour = Colour {
    r: 0.6588235294117647f32,
    g: 0.49019607843137253f32,
    b: 0.7607843137254902f32,
    a: 1f32,
};
pub const WisteriaBlue: Colour = Colour {
    r: 0.5176470588235295f32,
    g: 0.6352941176470588f32,
    b: 0.8313725490196079f32,
    a: 1f32,
};
pub const WitchBrew: Colour = Colour {
    r: 0.5333333333333333f32,
    g: 0.5294117647058824f32,
    b: 0.2196078431372549f32,
    a: 1f32,
};
pub const WitchHazel: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.9411764705882353f32,
    b: 0.45098039215686275f32,
    a: 1f32,
};
pub const Witchcraft: Colour = Colour {
    r: 0.2784313725490196f32,
    g: 0.2980392156862745f32,
    b: 0.3137254901960784f32,
    a: 1f32,
};
pub const Wizard: Colour = Colour {
    r: 0.30196078431372547f32,
    g: 0.3568627450980392f32,
    b: 0.5333333333333333f32,
    a: 1f32,
};
pub const WizardSBrew: Colour = Colour {
    r: 0.6274509803921569f32,
    g: 0.5647058823529412f32,
    b: 0.7215686274509804f32,
    a: 1f32,
};
pub const WolfPack: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.4666666666666667f32,
    b: 0.43529411764705883f32,
    a: 1f32,
};
pub const Wolfram: Colour = Colour {
    r: 0.7098039215686275f32,
    g: 0.7137254901960784f32,
    b: 0.7176470588235294f32,
    a: 1f32,
};
pub const WonderWine: Colour = Colour {
    r: 0.38823529411764707f32,
    g: 0.36470588235294116f32,
    b: 0.38823529411764707f32,
    a: 1f32,
};
pub const WondrousWisteria: Colour = Colour {
    r: 0.6392156862745098f32,
    g: 0.6941176470588235f32,
    b: 0.9490196078431372f32,
    a: 1f32,
};
pub const WoodBark: Colour = Colour {
    r: 0.18823529411764706f32,
    g: 0.14901960784313725f32,
    b: 0.12941176470588237f32,
    a: 1f32,
};
pub const Woodgrain: Colour = Colour {
    r: 0.6f32,
    g: 0.4f32,
    b: 0.2f32,
    a: 1f32,
};
pub const Woodhaven: Colour = Colour {
    r: 0.6196078431372549f32,
    g: 0.4823529411764706f32,
    b: 0.4235294117647059f32,
    a: 1f32,
};
pub const WoodlandGrass: Colour = Colour {
    r: 0.0f32,
    g: 0.26666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const WoodlandNight: Colour = Colour {
    r: 0.2784313725490196f32,
    g: 0.3607843137254902f32,
    b: 0.36470588235294116f32,
    a: 1f32,
};
pub const WoodlandSoul: Colour = Colour {
    r: 0.07058823529411765f32,
    g: 0.47843137254901963f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const WoodlandWonder: Colour = Colour {
    r: 0.050980392156862744f32,
    g: 0.38823529411764707f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const WorcestershireSauce: Colour = Colour {
    r: 0.3411764705882353f32,
    g: 0.16862745098039217f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
pub const WornSilver: Colour = Colour {
    r: 0.788235294117647f32,
    g: 0.7529411764705882f32,
    b: 0.7333333333333333f32,
    a: 1f32,
};
pub const WrappedInTwilight: Colour = Colour {
    r: 0.37254901960784315f32,
    g: 0.42745098039215684f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const Wreath: Colour = Colour {
    r: 0.4627450980392157f32,
    g: 0.5215686274509804f32,
    b: 0.41568627450980394f32,
    a: 1f32,
};
pub const WutangGold: Colour = Colour {
    r: 0.9725490196078431f32,
    g: 0.8196078431372549f32,
    b: 0.023529411764705882f32,
    a: 1f32,
};
pub const XMarksTheSpot: Colour = Colour {
    r: 0.9019607843137255f32,
    g: 0.2784313725490196f32,
    b: 0.2901960784313726f32,
    a: 1f32,
};
pub const Xanthic: Colour = Colour {
    r: 0.9568627450980393f32,
    g: 0.8862745098039215f32,
    b: 0.08627450980392157f32,
    a: 1f32,
};
pub const XmasCandy: Colour = Colour {
    r: 0.6f32,
    g: 0.0f32,
    b: 0.12549019607843137f32,
    a: 1f32,
};
pub const Xoxo: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.5176470588235295f32,
    b: 0.592156862745098f32,
    a: 1f32,
};
pub const YachtClub: Colour = Colour {
    r: 0.33725490196078434f32,
    g: 0.3764705882352941f32,
    b: 0.3843137254901961f32,
    a: 1f32,
};
pub const Yakitori: Colour = Colour {
    r: 0.9254901960784314f32,
    g: 0.6705882352941176f32,
    b: 0.24705882352941178f32,
    a: 1f32,
};
pub const YangMist: Colour = Colour {
    r: 0.9294117647058824f32,
    g: 0.9098039215686274f32,
    b: 0.8666666666666667f32,
    a: 1f32,
};
pub const YearningDesire: Colour = Colour {
    r: 0.792156862745098f32,
    g: 0.07450980392156863f32,
    b: 0.3686274509803922f32,
    a: 1f32,
};
pub const YellForYellow: Colour = Colour {
    r: 1.0f32,
    g: 0.996078431372549f32,
    b: 0.0f32,
    a: 1f32,
};
pub const YellYellow: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const Yellow: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const YellowBuzzing: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.8666666666666667f32,
    b: 0.06666666666666667f32,
    a: 1f32,
};
pub const YellowChalk: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9764705882352941f32,
    b: 0.6784313725490196f32,
    a: 1f32,
};
pub const YellowMana: Colour = Colour {
    r: 0.9921568627450981f32,
    g: 0.9882352941176471f32,
    b: 0.7490196078431373f32,
    a: 1f32,
};
pub const YellowMellow: Colour = Colour {
    r: 0.9411764705882353f32,
    g: 0.8274509803921568f32,
    b: 0.11764705882352941f32,
    a: 1f32,
};
pub const YellowSubmarine: Colour = Colour {
    r: 1.0f32,
    g: 1.0f32,
    b: 0.0784313725490196f32,
    a: 1f32,
};
pub const Yellowish: Colour = Colour {
    r: 0.9803921568627451f32,
    g: 0.9333333333333333f32,
    b: 0.4f32,
    a: 1f32,
};
pub const YetiFootprint: Colour = Colour {
    r: 0.7803921568627451f32,
    g: 0.8431372549019608f32,
    b: 0.8784313725490196f32,
    a: 1f32,
};
pub const YippieYaYellow: Colour = Colour {
    r: 0.9764705882352941f32,
    g: 0.9607843137254902f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const YoghurtBrûlée: Colour = Colour {
    r: 0.9607843137254902f32,
    g: 0.9137254901960784f32,
    b: 0.807843137254902f32,
    a: 1f32,
};
pub const YorkPink: Colour = Colour {
    r: 0.8431372549019608f32,
    g: 0.5137254901960784f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const YorkPlum: Colour = Colour {
    r: 0.8274509803921568f32,
    g: 0.7490196078431373f32,
    b: 0.8980392156862745f32,
    a: 1f32,
};
pub const YorkshireCloud: Colour = Colour {
    r: 0.7294117647058823f32,
    g: 0.7647058823529411f32,
    b: 0.8f32,
    a: 1f32,
};
pub const Yoshi: Colour = Colour {
    r: 0.3333333333333333f32,
    g: 0.6666666666666666f32,
    b: 0.0f32,
    a: 1f32,
};
pub const YoungApricot: Colour = Colour {
    r: 0.9882352941176471f32,
    g: 0.8470588235294118f32,
    b: 0.7098039215686275f32,
    a: 1f32,
};
pub const YoungCrab: Colour = Colour {
    r: 0.9647058823529412f32,
    g: 0.6274509803921569f32,
    b: 0.615686274509804f32,
    a: 1f32,
};
pub const YoungNight: Colour = Colour {
    r: 0.13725490196078433f32,
    g: 0.13725490196078433f32,
    b: 0.13725490196078433f32,
    a: 1f32,
};
pub const YoungSalmon: Colour = Colour {
    r: 1.0f32,
    g: 0.7137254901960784f32,
    b: 0.7058823529411765f32,
    a: 1f32,
};
pub const YourDarkness: Colour = Colour {
    r: 0.13333333333333333f32,
    g: 0.0f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const YourMajesty: Colour = Colour {
    r: 0.3803921568627451f32,
    g: 0.28627450980392155f32,
    b: 0.43137254901960786f32,
    a: 1f32,
};
pub const YourShadow: Colour = Colour {
    r: 0.47058823529411764f32,
    g: 0.49411764705882355f32,
    b: 0.5764705882352941f32,
    a: 1f32,
};
pub const YoursTruly: Colour = Colour {
    r: 0.984313725490196f32,
    g: 0.8509803921568627f32,
    b: 0.803921568627451f32,
    a: 1f32,
};
pub const Yucca: Colour = Colour {
    r: 0.4588235294117647f32,
    g: 0.592156862745098f32,
    b: 0.5607843137254902f32,
    a: 1f32,
};
pub const YumaGold: Colour = Colour {
    r: 1.0f32,
    g: 0.8392156862745098f32,
    b: 0.47058823529411764f32,
    a: 1f32,
};
pub const YuzuMarmalade: Colour = Colour {
    r: 1.0f32,
    g: 0.8431372549019608f32,
    b: 0.4f32,
    a: 1f32,
};
pub const Yuzukoshō: Colour = Colour {
    r: 0.8313725490196079f32,
    g: 0.8705882352941177f32,
    b: 0.28627450980392155f32,
    a: 1f32,
};
pub const Zen: Colour = Colour {
    r: 0.8117647058823529f32,
    g: 0.8509803921568627f32,
    b: 0.8705882352941177f32,
    a: 1f32,
};
pub const ZenGarden: Colour = Colour {
    r: 0.8196078431372549f32,
    g: 0.8549019607843137f32,
    b: 0.7529411764705882f32,
    a: 1f32,
};
pub const Zenith: Colour = Colour {
    r: 0.28627450980392155f32,
    g: 0.47843137254901963f32,
    b: 0.6235294117647059f32,
    a: 1f32,
};
pub const ZeusSBolt: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 1.0f32,
    b: 0.0f32,
    a: 1f32,
};
pub const Zinc: Colour = Colour {
    r: 0.5725490196078431f32,
    g: 0.5372549019607843f32,
    b: 0.5411764705882353f32,
    a: 1f32,
};
pub const ZodiacConstellation: Colour = Colour {
    r: 0.9333333333333333f32,
    g: 0.5333333333333333f32,
    b: 0.26666666666666666f32,
    a: 1f32,
};
pub const Zombie: Colour = Colour {
    r: 0.41568627450980394f32,
    g: 0.4588235294117647f32,
    b: 0.35294117647058826f32,
    a: 1f32,
};
pub const Zoodles: Colour = Colour {
    r: 0.7215686274509804f32,
    g: 0.7490196078431373f32,
    b: 0.44313725490196076f32,
    a: 1f32,
};
pub const Zucchini: Colour = Colour {
    r: 0.09019607843137255f32,
    g: 0.27450980392156865f32,
    b: 0.1803921568627451f32,
    a: 1f32,
};
pub const ZucchiniNoodles: Colour = Colour {
    r: 0.7843137254901961f32,
    g: 0.8156862745098039f32,
    b: 0.4980392156862745f32,
    a: 1f32,
};
pub const ZundaGreen: Colour = Colour {
    r: 0.4196078431372549f32,
    g: 0.7529411764705882f32,
    b: 0.14901960784313725f32,
    a: 1f32,
};
