pub mod a_command_parser;
pub mod blank_line_remover;
pub mod c_command_parser;
pub mod command_parser;
pub mod comment_remover;
pub mod file_reader;
pub mod hack_file_writer;
pub mod line_parser;
pub mod symbol_parser;
pub mod whitespace_remover;

fn main() {
    println!("Hello, world!");
}

/*
 * TODO: issiaiskinti, prisiminti ka cia reikia padaryti.
 * reikia prisiminti ka as cia isvis darau.
 * kaip parsinu, ka cia parsinu, reiktu pavyzduku pasiziureti. atsisiuti kad doca, kuris
 *
 */
// asssemberis fzf
// kas turi but padaryta
//
// moduliai:
// DONE     - skaityti faila -> output tekstinis blokas - string[]
//      - parsinti kiekviena eilute, ignoruojant komentarus/whitespace -> string[]
//      - nuspresti kokia tai komanda, A ar C -> grazinti struck kuris turetu initial komandas A ir C
//      - parsinti kiekviena komanda is eiles A arba C
//      - TODO: simboliai, linkai

/*
 * reikia perskaityiti kiekvina eilute
 * - ignoruoti empty lines
 * - ignoruoti komentarus
 *
 *
 * kiekviena eilute isparsinti pagal komandos tipa:
 * - prasideda @:
 * -- A instrukcija:
 *      pasiimti kas eina po @ ir paversti i binary
 *
 * -- NEprasideda @:
 * -- C instrucija:
 *      isskaidyti kiekviena dali atskirai i jos komponentus TODO
 */
