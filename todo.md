ka cia reikia padaryti su assembleriu:

- DONE - perskaityti faila, nurodant file path 
- DONE - perskaityti kiekviena eilute is failo, ir susideti i lista
- DONE - istrinti visus komentarus, jeigu prasideda du `//` tada viska isstrinti
- DONE - isstrinti visus white spaces
--------------------------------------------------
- DONE - isstrinti visas empty line
--------------------------------------------------

# Komandu parsinimas

--- komandos @ parsinimas
- paziureti ar prasideda komanda su `@` ->
- paversti ta skaiciu einanti po `@` i binary
- issaugoti ji kaip `1 + binary skaicius (padded to n=15)` // op code = 1

// saugoti abidvi komanadas i enuma. ir abudu enumai tures `fn toMachine() -> String`


### NO symbols
- paziureti kaip prasideda komanda, ir isparsinti i A ir C komandas
    - A komands: jei prasideda `@`
    tiesiog skaiciu paversti i dvejetaini ir prideti nulius
    toMachineCommand() -> 1 + `binary skaicius is komandos`

    - C komands: nei ne prasideda @?
    jeigu yra `=` tada reiskia bus DEST -> priskirti is ENUM DEST value / arba null
    tada bus COMP -> priskirti is ENUM COMP value
    jeigu yra `;` tada bus JUMP -> priskirti is ENUM JUMP value / arba null
    - kiekvienas is situ tures savo binary value ENUMe
    toMAchineCommand() -> opCode + 111 + DEST.binary + COMP.binary + JMP.binary

- praeiti kiekviena komanda ir paversti i machineCommand -> machineCommand[]
- pereiti pro mCmd[] ir surasyti i output faila `hack.asm` ?

### WITH symbols

- tureti MAP, kuris butu symbols table. Prideti default values i ji
- pereiti per visas eilutes, ir sunumeruoti jas ( iskyrus LABELS ) linked list?
- pereiti per visas eilutes ir visus (LABELS) sudeti i symbols table
- pereiti per visus ir @variable sudeti i symbols table -> nuo n=16
- pereiti per visus ir kiekviena symboli pakeisti is lookup table

-? kaskaip reikia tuos labelius irgi istrinti. arba tureti atskira komandu lista, kur yra labeliai ir kur nera

-> goto `NO symbols`


