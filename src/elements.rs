use std::ops::Index;

use crate::prelude::*;

/// Materials for representations of the elements
///
/// May be indexed by atomic number. Index 0 is reserved for
/// virtual sites or dummy atoms.
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

impl Index<usize> for ElementMaterials {
    type Output = Handle<StandardMaterial>;

    fn index(&self, index: usize) -> &Self::Output {
        Index::index(&self.0, index)
    }
}

/// Colours for representations of the elements.
///
/// This array may be indexed by atomic number. Index 0 is reserved for virtual
/// sites or dummy atoms.
const ELEMENTCOLOURS: [Color; 119] = [
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

/// Van der Waals radii of the elements.
///
/// This array may be indexed by atomic number. Index 0 is reserved for virtual
/// sites or dummy atoms and returns a value of 1.0.
///
/// Elements 1-60, 62-83, and 89-99 are taken from
/// > "A cartography of the van der Waals territories"\
/// > Santiago Alvarez\
/// > Dalton Trans., 2013, 42, 8617-8636\
/// > DOI: [10.1039/C3DT50599E](https://doi.org/10.1039/C3DT50599E)
///
/// Elements 84-88 are taken from
/// > "Consistent van der Waals Radii for the Whole Main Group"\
/// > Manjeera Mantina, Adam C. Chamberlin, Rosendo Valero, Christopher J. Cramer, and Donald G. Truhlar\
/// > J. Phys. Chem. A, 2009, 113 (19), 5806-5812\
/// > DOI: [10.1021/jp8111556](https://doi.org/10.1021/jp8111556)
///
/// Elements 61 and 100+ are estimated to 1 significant figure from surrounding trends.
///
/// A sensible alternative for elements 1-96 would be
/// the atomic radii from
/// > "Atomic and Ionic Radii of Elements 1-96"\
/// > Martin Rahm, Roald Hoffman, and N. W. Ashcroft\
/// > Chemistry - A European Journal, 2016, 22 (41), 14625-14632\
/// > DOI: [10.1002/chem.201602949](https://doi.org/10.1002/chem.201602949)
pub const ELEMENTRADII: [f32; 119] = [
    1.0,  // Virtual sites/dummy atoms
    1.20, // Hydrogen,H,1
    1.43, // Helium,He,2
    2.12, // Lithium,Li,3
    1.98, // Beryllium,Be,4
    1.91, // Boron,B,5
    1.77, // Carbon,C,6
    1.66, // Nitrogen,N,7
    1.50, // Oxygen,O,8
    1.46, // Fluorine,F,9
    1.58, // Neon,Ne,10
    2.50, // Sodium,Na,11
    2.51, // Magnesium,Mg,12
    2.25, // Aluminium,Al,13
    2.19, // Silicon,Si,14
    1.90, // Phosphorus,P,15
    1.89, // Sulfur,S,16
    1.82, // Chlorine,Cl,17
    1.83, // Argon,Ar,18
    2.73, // Potassium,K,19
    2.62, // Calcium,Ca,20
    2.58, // Scandium,Sc,21
    2.46, // Titanium,Ti,22
    2.42, // Vanadium,V,23
    2.45, // Chromium,Cr,24
    2.45, // Manganese,Mn,25
    2.44, // Iron,Fe,26
    2.40, // Cobalt,Co,27
    2.40, // Nickel,Ni,28
    2.38, // Copper,Cu,29
    2.39, // Zinc,Zn,30
    2.32, // Gallium,Ga,31
    2.29, // Germanium,Ge,32
    1.88, // Arsenic,As,33
    1.82, // Selenium,Se,34
    1.86, // Bromine,Br,35
    2.25, // Krypton,Kr,36
    3.21, // Rubidium,Rb,37
    2.84, // Strontium,Sr,38
    2.75, // Yttrium,Y,39
    2.52, // Zirconium,Zr,40
    2.56, // Niobium,Nb,41
    2.45, // Molybdenum,Mo,42
    2.44, // Technetium,Tc,43
    2.46, // Ruthenium,Ru,44
    2.44, // Rhodium,Rh,45
    2.15, // Palladium,Pd,46
    2.53, // Silver,Ag,47
    2.49, // Cadmium,Cd,48
    2.43, // Indium,In,49
    2.42, // Tin,Sn,50
    2.47, // Antimony,Sb,51
    1.99, // Tellurium,Te,52
    2.04, // Iodine,I,53
    2.06, // Xenon,Xe,54
    3.48, // Caesium,Cs,55
    3.03, // Barium,Ba,56
    2.98, // Lanthanum,La,57
    2.88, // Cerium,Ce,58
    2.92, // Praseodymium,Pr,59
    2.95, // Neodymium,Nd,60
    3.,   // Promethium,Pm,61
    2.90, // Samarium,Sm,62
    2.87, // Europium,Eu,63
    2.83, // Gadolinium,Gd,64
    2.79, // Terbium,Tb,65
    2.87, // Dysprosium,Dy,66
    2.81, // Holmium,Ho,67
    2.83, // Erbium,Er,68
    2.79, // Thulium,Tm,69
    2.80, // Ytterbium,Yb,70
    2.74, // Lutetium,Lu,71
    2.63, // Hafnium,Hf,72
    2.53, // Tantalum,Ta,73
    2.57, // Tungsten,W,74
    2.49, // Rhenium,Re,75
    2.48, // Osmium,Os,76
    2.41, // Iridium,Ir,77
    2.29, // Platinum,Pt,78
    2.32, // Gold,Au,79
    2.45, // Mercury,Hg,80
    2.47, // Thallium,Tl,81
    2.60, // Lead,Pb,82
    2.54, // Bismuth,Bi,83
    1.97, // Polonium,Po,84
    2.02, // Astatine,At,85
    2.20, // Radon,Rn,86
    3.48, // Francium,Fr,87
    2.83, // Radium,Ra,88
    2.8,  // Actinium,Ac,89
    2.93, // Thorium,Th,90
    2.88, // Protactinium,Pa,91
    2.71, // Uranium,U,92
    2.82, // Neptunium,Np,93
    2.81, // Plutonium,Pu,94
    2.83, // Americium,Am,95
    3.05, // Curium,Cm,96
    3.4,  // Berkelium,Bk,97
    3.05, // Californium,Cf,98
    2.7,  // Einsteinium,Es,99
    3.,   // Fermium,Fm,100
    3.,   // Mendelevium,Md,101
    3.,   // Nobelium,No,102
    3.,   // Lawrencium,Lr,103
    3.,   // Rutherfordium,Rf,104
    3.,   // Dubnium,Db,105
    3.,   // Seaborgium,Sg,106
    3.,   // Bohrium,Bh,107
    3.,   // Hassium,Hs,108
    3.,   // Meitnerium,Mt,109
    3.,   // Darmstadtium,Ds,110
    3.,   // Roentgenium,Rg,111
    3.,   // Copernicium,Cn,112
    3.,   // Nihonium,Nh,113
    3.,   // Flerovium,Fl,114
    3.,   // Moscovium,Mc,115
    3.,   // Livermorium,Lv,116
    3.,   // Tennessine,Ts,117
    3.,   // Oganesson,Og,118
];

/// Names of the elements.
///
/// This array may be indexed by atomic number. Index 0 is reserved for virtual
/// sites or dummy atoms and has the value "VirtualSite"
pub const ELEMENTNAMES: [&'static str; 119] = [
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
