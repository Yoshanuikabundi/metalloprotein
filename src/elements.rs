use crate::prelude::*;

pub struct ElementMaterials(pub [Handle<StandardMaterial>; 119]);

impl FromWorld for ElementMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .unwrap();
        let buf = ELEMENTCOLOURS.map(|color| materials.add(color.into()));
        Self(buf)
    }
}

static ELEMENTCOLOURS: [Color; 119] = [
    Color::ANTIQUE_WHITE, // Virtual sites/dummy atoms
    Color::WHITE,         // Hydrogen,H,1
    Color::BLACK,         // Helium,He,2
    Color::BLACK,         // Lithium,Li,3
    Color::BLACK,         // Beryllium,Be,4
    Color::BLACK,         // Boron,B,5
    Color::DARK_GRAY,     // Carbon,C,6
    Color::BLUE,          // Nitrogen,N,7
    Color::RED,           // Oxygen,O,8
    Color::BLACK,         // Fluorine,F,9
    Color::BLACK,         // Neon,Ne,10
    Color::BLACK,         // Sodium,Na,11
    Color::BLACK,         // Magnesium,Mg,12
    Color::BLACK,         // Aluminium,Al,13
    Color::BLACK,         // Silicon,Si,14
    Color::BLACK,         // Phosphorus,P,15
    Color::YELLOW,        // Sulfur,S,16
    Color::GREEN,         // Chlorine,Cl,17
    Color::BLACK,         // Argon,Ar,18
    Color::BLACK,         // Potassium,K,19
    Color::BLACK,         // Calcium,Ca,20
    Color::BLACK,         // Scandium,Sc,21
    Color::BLACK,         // Titanium,Ti,22
    Color::BLACK,         // Vanadium,V,23
    Color::BLACK,         // Chromium,Cr,24
    Color::BLACK,         // Manganese,Mn,25
    Color::BLACK,         // Iron,Fe,26
    Color::BLACK,         // Cobalt,Co,27
    Color::BLACK,         // Nickel,Ni,28
    Color::BLACK,         // Copper,Cu,29
    Color::BLACK,         // Zinc,Zn,30
    Color::BLACK,         // Gallium,Ga,31
    Color::BLACK,         // Germanium,Ge,32
    Color::BLACK,         // Arsenic,As,33
    Color::BLACK,         // Selenium,Se,34
    Color::BLACK,         // Bromine,Br,35
    Color::BLACK,         // Krypton,Kr,36
    Color::BLACK,         // Rubidium,Rb,37
    Color::BLACK,         // Strontium,Sr,38
    Color::BLACK,         // Yttrium,Y,39
    Color::BLACK,         // Zirconium,Zr,40
    Color::BLACK,         // Niobium,Nb,41
    Color::BLACK,         // Molybdenum,Mo,42
    Color::BLACK,         // Technetium,Tc,43
    Color::BLACK,         // Ruthenium,Ru,44
    Color::BLACK,         // Rhodium,Rh,45
    Color::BLACK,         // Palladium,Pd,46
    Color::BLACK,         // Silver,Ag,47
    Color::BLACK,         // Cadmium,Cd,48
    Color::BLACK,         // Indium,In,49
    Color::BLACK,         // Tin,Sn,50
    Color::BLACK,         // Antimony,Sb,51
    Color::BLACK,         // Tellurium,Te,52
    Color::BLACK,         // Iodine,I,53
    Color::BLACK,         // Xenon,Xe,54
    Color::BLACK,         // Caesium,Cs,55
    Color::BLACK,         // Barium,Ba,56
    Color::BLACK,         // Lanthanum,La,57
    Color::BLACK,         // Cerium,Ce,58
    Color::BLACK,         // Praseodymium,Pr,59
    Color::BLACK,         // Neodymium,Nd,60
    Color::BLACK,         // Promethium,Pm,61
    Color::BLACK,         // Samarium,Sm,62
    Color::BLACK,         // Europium,Eu,63
    Color::BLACK,         // Gadolinium,Gd,64
    Color::BLACK,         // Terbium,Tb,65
    Color::BLACK,         // Dysprosium,Dy,66
    Color::BLACK,         // Holmium,Ho,67
    Color::BLACK,         // Erbium,Er,68
    Color::BLACK,         // Thulium,Tm,69
    Color::BLACK,         // Ytterbium,Yb,70
    Color::BLACK,         // Lutetium,Lu,71
    Color::BLACK,         // Hafnium,Hf,72
    Color::BLACK,         // Tantalum,Ta,73
    Color::BLACK,         // Tungsten,W,74
    Color::BLACK,         // Rhenium,Re,75
    Color::BLACK,         // Osmium,Os,76
    Color::BLACK,         // Iridium,Ir,77
    Color::BLACK,         // Platinum,Pt,78
    Color::BLACK,         // Gold,Au,79
    Color::BLACK,         // Mercury,Hg,80
    Color::BLACK,         // Thallium,Tl,81
    Color::BLACK,         // Lead,Pb,82
    Color::BLACK,         // Bismuth,Bi,83
    Color::BLACK,         // Polonium,Po,84
    Color::BLACK,         // Astatine,At,85
    Color::BLACK,         // Radon,Rn,86
    Color::BLACK,         // Francium,Fr,87
    Color::BLACK,         // Radium,Ra,88
    Color::BLACK,         // Actinium,Ac,89
    Color::BLACK,         // Thorium,Th,90
    Color::BLACK,         // Protactinium,Pa,91
    Color::BLACK,         // Uranium,U,92
    Color::BLACK,         // Neptunium,Np,93
    Color::BLACK,         // Plutonium,Pu,94
    Color::BLACK,         // Americium,Am,95
    Color::BLACK,         // Curium,Cm,96
    Color::BLACK,         // Berkelium,Bk,97
    Color::BLACK,         // Californium,Cf,98
    Color::BLACK,         // Einsteinium,Es,99
    Color::BLACK,         // Fermium,Fm,100
    Color::BLACK,         // Mendelevium,Md,101
    Color::BLACK,         // Nobelium,No,102
    Color::BLACK,         // Lawrencium,Lr,103
    Color::BLACK,         // Rutherfordium,Rf,104
    Color::BLACK,         // Dubnium,Db,105
    Color::BLACK,         // Seaborgium,Sg,106
    Color::BLACK,         // Bohrium,Bh,107
    Color::BLACK,         // Hassium,Hs,108
    Color::BLACK,         // Meitnerium,Mt,109
    Color::BLACK,         // Darmstadtium,Ds,110
    Color::BLACK,         // Roentgenium,Rg,111
    Color::BLACK,         // Copernicium,Cn,112
    Color::BLACK,         // Nihonium,Nh,113
    Color::BLACK,         // Flerovium,Fl,114
    Color::BLACK,         // Moscovium,Mc,115
    Color::BLACK,         // Livermorium,Lv,116
    Color::BLACK,         // Tennessine,Ts,117
    Color::BLACK,         // Oganesson,Og,118
];

/// From Consistent van der Waals Radii for the Whole Main Group
/// Manjeera Mantina, Adam C. Chamberlin, Rosendo Valero, Christopher J. Cramer, and Donald G. Truhlar
/// The Journal of Physical Chemistry A 2009 113 (19), 5806-5812
/// DOI: [10.1021/jp8111556](https://doi.org/10.1021/jp8111556)
pub static ELEMENTRADII: [f32; 119] = [
    1.0,  // Virtual sites/dummy atoms
    1.10, // Hydrogen,H,1
    1.40, // Helium,He,2
    1.81, // Lithium,Li,3
    1.53, // Beryllium,Be,4
    1.92, // Boron,B,5
    1.70, // Carbon,C,6
    1.55, // Nitrogen,N,7
    1.52, // Oxygen,O,8
    1.47, // Fluorine,F,9
    1.54, // Neon,Ne,10
    2.27, // Sodium,Na,11
    1.73, // Magnesium,Mg,12
    1.84, // Aluminium,Al,13
    2.10, // Silicon,Si,14
    1.80, // Phosphorus,P,15
    1.80, // Sulfur,S,16
    1.75, // Chlorine,Cl,17
    1.88, // Argon,Ar,18
    2.75, // Potassium,K,19
    2.31, // Calcium,Ca,20
    1.0,  // Scandium,Sc,21
    1.0,  // Titanium,Ti,22
    1.0,  // Vanadium,V,23
    1.0,  // Chromium,Cr,24
    1.0,  // Manganese,Mn,25
    1.0,  // Iron,Fe,26
    1.0,  // Cobalt,Co,27
    1.0,  // Nickel,Ni,28
    1.0,  // Copper,Cu,29
    1.0,  // Zinc,Zn,30
    1.87, // Gallium,Ga,31
    2.11, // Germanium,Ge,32
    1.85, // Arsenic,As,33
    1.90, // Selenium,Se,34
    1.83, // Bromine,Br,35
    2.02, // Krypton,Kr,36
    3.03, // Rubidium,Rb,37
    2.49, // Strontium,Sr,38
    1.0,  // Yttrium,Y,39
    1.0,  // Zirconium,Zr,40
    1.0,  // Niobium,Nb,41
    1.0,  // Molybdenum,Mo,42
    1.0,  // Technetium,Tc,43
    1.0,  // Ruthenium,Ru,44
    1.0,  // Rhodium,Rh,45
    1.0,  // Palladium,Pd,46
    1.0,  // Silver,Ag,47
    1.0,  // Cadmium,Cd,48
    1.93, // Indium,In,49
    2.17, // Tin,Sn,50
    2.06, // Antimony,Sb,51
    2.06, // Tellurium,Te,52
    1.98, // Iodine,I,53
    2.16, // Xenon,Xe,54
    3.43, // Caesium,Cs,55
    2.68, // Barium,Ba,56
    1.0,  // Lanthanum,La,57
    1.0,  // Cerium,Ce,58
    1.0,  // Praseodymium,Pr,59
    1.0,  // Neodymium,Nd,60
    1.0,  // Promethium,Pm,61
    1.0,  // Samarium,Sm,62
    1.0,  // Europium,Eu,63
    1.0,  // Gadolinium,Gd,64
    1.0,  // Terbium,Tb,65
    1.0,  // Dysprosium,Dy,66
    1.0,  // Holmium,Ho,67
    1.0,  // Erbium,Er,68
    1.0,  // Thulium,Tm,69
    1.0,  // Ytterbium,Yb,70
    1.0,  // Lutetium,Lu,71
    1.0,  // Hafnium,Hf,72
    1.0,  // Tantalum,Ta,73
    1.0,  // Tungsten,W,74
    1.0,  // Rhenium,Re,75
    1.0,  // Osmium,Os,76
    1.0,  // Iridium,Ir,77
    1.0,  // Platinum,Pt,78
    1.0,  // Gold,Au,79
    1.0,  // Mercury,Hg,80
    1.96, // Thallium,Tl,81
    2.02, // Lead,Pb,82
    2.07, // Bismuth,Bi,83
    1.97, // Polonium,Po,84
    2.02, // Astatine,At,85
    2.20, // Radon,Rn,86
    3.48, // Francium,Fr,87
    2.83, // Radium,Ra,88
    1.0,  // Actinium,Ac,89
    1.0,  // Thorium,Th,90
    1.0,  // Protactinium,Pa,91
    1.0,  // Uranium,U,92
    1.0,  // Neptunium,Np,93
    1.0,  // Plutonium,Pu,94
    1.0,  // Americium,Am,95
    1.0,  // Curium,Cm,96
    1.0,  // Berkelium,Bk,97
    1.0,  // Californium,Cf,98
    1.0,  // Einsteinium,Es,99
    1.0,  // Fermium,Fm,100
    1.0,  // Mendelevium,Md,101
    1.0,  // Nobelium,No,102
    1.0,  // Lawrencium,Lr,103
    1.0,  // Rutherfordium,Rf,104
    1.0,  // Dubnium,Db,105
    1.0,  // Seaborgium,Sg,106
    1.0,  // Bohrium,Bh,107
    1.0,  // Hassium,Hs,108
    1.0,  // Meitnerium,Mt,109
    1.0,  // Darmstadtium,Ds,110
    1.0,  // Roentgenium,Rg,111
    1.0,  // Copernicium,Cn,112
    1.0,  // Nihonium,Nh,113
    1.0,  // Flerovium,Fl,114
    1.0,  // Moscovium,Mc,115
    1.0,  // Livermorium,Lv,116
    1.0,  // Tennessine,Ts,117
    1.0,  // Oganesson,Og,118
];

pub static ELEMENTNAMES: [&'static str; 119] = [
    "VirtualSite",
    "Hydrogen",
    "Helium",
    "Lithium",
    "Beryllium",
    "Boron",
    "Carbon",
    "Nitrogen",
    "Oxygen",
    "Fluorine",
    "Neon",
    "Sodium",
    "Magnesium",
    "Aluminium",
    "Silicon",
    "Phosphorus",
    "Sulfur",
    "Chlorine",
    "Argon",
    "Potassium",
    "Calcium",
    "Scandium",
    "Titanium",
    "Vanadium",
    "Chromium",
    "Manganese",
    "Iron",
    "Cobalt",
    "Nickel",
    "Copper",
    "Zinc",
    "Gallium",
    "Germanium",
    "Arsenic",
    "Selenium",
    "Bromine",
    "Krypton",
    "Rubidium",
    "Strontium",
    "Yttrium",
    "Zirconium",
    "Niobium",
    "Molybdenum",
    "Technetium",
    "Ruthenium",
    "Rhodium",
    "Palladium",
    "Silver",
    "Cadmium",
    "Indium",
    "Tin",
    "Antimony",
    "Tellurium",
    "Iodine",
    "Xenon",
    "Caesium",
    "Barium",
    "Lanthanum",
    "Cerium",
    "Praseodymium",
    "Neodymium",
    "Promethium",
    "Samarium",
    "Europium",
    "Gadolinium",
    "Terbium",
    "Dysprosium",
    "Holmium",
    "Erbium",
    "Thulium",
    "Ytterbium",
    "Lutetium",
    "Hafnium",
    "Tantalum",
    "Tungsten",
    "Rhenium",
    "Osmium",
    "Iridium",
    "Platinum",
    "Gold",
    "Mercury",
    "Thallium",
    "Lead",
    "Bismuth",
    "Polonium",
    "Astatine",
    "Radon",
    "Francium",
    "Radium",
    "Actinium",
    "Thorium",
    "Protactinium",
    "Uranium",
    "Neptunium",
    "Plutonium",
    "Americium",
    "Curium",
    "Berkelium",
    "Californium",
    "Einsteinium",
    "Fermium",
    "Mendelevium",
    "Nobelium",
    "Lawrencium",
    "Rutherfordium",
    "Dubnium",
    "Seaborgium",
    "Bohrium",
    "Hassium",
    "Meitnerium",
    "Darmstadtium",
    "Roentgenium",
    "Copernicium",
    "Nihonium",
    "Flerovium",
    "Moscovium",
    "Livermorium",
    "Tennessine",
    "Oganesson",
];
