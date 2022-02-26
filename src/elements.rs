use bevy::prelude::*;

pub(crate) struct ElementMaterials(pub(crate) [Handle<StandardMaterial>; 119]);

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
    Color::WHITE,         // Virtual sites/dummy atoms
    Color::ANTIQUE_WHITE, // Hydrogen,H,1
    Color::BLACK,         // Helium,He,2
    Color::BLACK,         // Lithium,Li,3
    Color::BLACK,         // Beryllium,Be,4
    Color::BLACK,         // Boron,B,5
    Color::GRAY,          // Carbon,C,6
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
