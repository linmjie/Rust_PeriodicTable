use std::io;
use std::collections::HashMap;
use std::fmt;

macro_rules! makeElement {
        ($name: ident, $mass: literal, $number: literal ) => {
            let $name = Element {
                name: stringify!($name).to_string(),
                mass: $mass,
                number: $number
            };
        };
}

macro_rules! periodicHelper {
    ($table: ident, $symbol: ident, $name: ident, $mass: literal, $number: literal) => {
             makeElement!($name, $mass, $number);
             $table.insert(stringify!($symbol), $name);
    };
    ($table: ident, $symbol: ident, $name: ident, $mass: literal, $number: literal,
        $($symbolx: ident, $namex: ident, $massx: literal, $numberx: literal),+) => {{
            periodicHelper!($table, $symbol, $name, $mass, $number);
            periodicHelper!($table, $($symbolx, $namex, $massx, $numberx),+);
    }};

}

macro_rules! makePeriodicTable {
         ($table: ident {
             $symbol: ident, $name: ident, $mass: literal, $number: literal
         }) => {
             let mut $table: HashMap<&str, Element> = HashMap::new();
             periodicHelper!($table, $symbol, $name, $mass, $number);
        };

         ($table: ident {
             $symbol: ident, $name: ident, $mass: literal, $number: literal,
             $($symbolx: ident, $namex: ident, $massx: literal, $numberx: literal),+
         }) => {
             let mut $table: HashMap<&str, Element> = HashMap::new();
             periodicHelper!($table, $symbol, $name, $mass, $number, $($symbolx, $namex, $massx, $numberx),+);
         };
}

fn main() {
    makePeriodicTable!(
        periodic_table {
            H, hydrogen, 1.00794, 1,
            He, helium, 4.00260, 2,
            Li, lithium, 6.941, 3,
            Be, beryllium, 9.012182, 4,
            B, boron, 10.81, 5,
            C, carbon, 12.0111, 6,
            N, nitrogen, 14.0067, 7,
            O, oxygen, 15.9994, 8,
            F, fluorine, 18.9984032, 9,
            Ne, neon, 20.1797, 10,
            Na, sodium, 22.98977, 11,
            Mg, magnesium, 24.305, 12,
            Al, aluminum, 26.9815154, 13,
            Si, silicon, 28.0855, 14,
            P, phosphorus, 30.973756, 15,
            S, sulfur, 32.06, 16,
            Cl, chlorine, 35.453, 17,
            Ar, argon, 39.948, 18,
            K, potassium, 39.0983, 19,
            Ca, calcium, 40.08, 20,
            Sc, scandium, 44.9559, 21,
            Ti, titanium, 47.88, 22,
            V, vanadium, 50.9415, 23,
            Cr, chromium, 51.996, 24,
            Mn, manganese, 54.9380, 25,
            Fe, iron, 55.847, 26,
            Co, cobalt, 58.9332, 27,
            Ni, nickel, 58.69, 28,
            Cu, copper, 63.546, 29,
            Zn, zinc, 65.39, 30,
            Ga, gallium, 69.72, 31,
            Ge, germanium, 72.59, 32,
            As, arsenic, 74.9216, 33,
            Se, selenium, 78.96, 34,
            Br, bromine, 79.904, 35,
            Kr, krypton, 83.80, 36,
            Rb, rubidium, 85.4678, 37,
            Sr, strontium, 87.62, 38,
            Y, yttrium, 88.9059, 39,
            Zr, zirconium, 91.224, 40,
            Nb, niobium, 92.9064, 41,
            Mo, molybdenum, 95.94, 42,
            Tc, technetium, 98.0, 43,
            Ru, ruthenium, 101.07, 44,
            Rh, rhodium, 102.906, 45,
            Pd, palladium, 106.42, 46,
            Ag, silver, 107.868, 47,
            Cd, cadmium, 112.41, 48,
            In, indium, 114.82, 49,
            Sn, tin, 118.71, 50,
            Sb, antimony, 121.75, 51,
            Te, tellurium, 127.60, 52,
            I, iodine, 126.905, 53,
            Xe, xenon, 131.29, 54,
            Cs, cesium, 132.905, 55,
            Ba, barium, 137.33, 56,
            La, lanthanum, 138.906, 57,
            Ce, cerium, 140.12, 58,
            Pr, praseodymium, 140.908, 59,
            Nd, neodymium, 144.24, 60,
            Pm, promethium, 145.0, 61,
            Sm, samarium, 150.36, 62,
            Eu, europium, 151.96, 63,
            Gd, gadolinium, 157.25, 64,
            Tb, terbium, 158.925, 65,
            Dy, dysprosium, 162.50, 66,
            Ho, holmium, 164.9390, 67,
            Er, erbium, 167.26, 68,
            Tm, thulium, 168.934, 69,
            Yb, ytterbium, 173.04, 70,
            Lu, lutetium, 174.967, 71,
            Hf, hafnium, 178.49, 72,
            Ta, tantalum, 180.948, 73,
            W, tungsten, 183.85, 74,
            Re, rhenium, 186.207, 75,
            Os, osmium, 190.2, 76,
            Ir, iridium, 192.22, 77,
            Pt, platinum, 195.08, 78,
            Au, gold, 196.967, 79,
            Hg, mercury, 200.59, 80,
            Tl, thallium, 204.383, 81,
            Pb, lead, 207.2, 82,
            Bi, bismuth, 208.980, 83,
            Po, polonium, 209.0, 84,
            At, astatine, 210.0, 85,
            Rn, radon, 222.0, 86,
            Fr, francium, 223.0, 87,
            Ra, radium, 226.025, 88,
            Ac, actinium, 227.028, 89,
            Th, thorium, 232.038, 90,
            Pa, protactinium, 231.036, 91,
            U, uranium, 238.029, 92,
            Np, neptunium, 237.048, 93,
            Pu, plutonium, 244.0, 94,
            Am, americium, 243.0, 95,
            Cm, curium, 247.0, 96,
            Bk, berkelium, 247.0, 97,
            Cf, californium, 251.0, 98,
            Es, einsteinium, 252.0, 99,
            Fm, fermium, 257.0, 100,
            Md, mendelevium, 258.0, 101,
            No, nobelium, 259.0, 102,
            Lr, lawrencium, 262.0, 103,
            Rf, rutherfordium, 261.0, 104,
            Db, dubnium, 262.0, 105,
            Sg, seaborgium, 263.0, 106,
            Bh, bohrium, 262.0, 107,
            Hs, hassium, 265.0, 108,
            Mt, meitnerium, 266.0, 109
        }
    );

    loop {
        let mut input = String::new();
        io::stdin().read_line(& mut input).unwrap();

        match periodic_table.get(input.trim()){
            Some(val) => println!("{}", val),
            None => println!("Not an element!")
        }
    }
}

struct Element {
    name: String,
    mass: f32,
    number: i32
}

impl fmt::Display for Element {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:\n\tAtomic Number: {}\n\tAtomic Mass: {}",
            self.name.to_uppercase(), self.number, self.mass)
    }
}
