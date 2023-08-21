ka cia reikia padaryti su assembleriu:

- DONE - perskaityti faila, nurodant file path 
- DONE - perskaityti kiekviena eilute is failo, ir susideti i lista
- DONE - istrinti visus komentarus, jeigu prasideda du `//` tada viska isstrinti
- DONE - isstrinti visus white spaces
- DONE - isstrinti visas empty line

# Komandu parsinimas

- DONE --- komandos @ parsinimas
- DONE paziureti ar prasideda komanda su `@` ->
- DONE paversti ta skaiciu einanti po `@` i binary
- DONE issaugoti ji kaip `1 + binary skaicius (padded to n=15)` // op code = 1

--- C komandos parsinimas
- jeigu neprasideda `@` -> tada bus C komanda 
- OP code = `1` 
- +filleris `11`
--------------------------------------------------
- pradzioj - paprasta `COMP` komanda -> priskirti i enum value
--------------------------------------------------
- jeigu yra `=` -> isparsinti `DEST` ir priskirti i enum value
- jeigu yra `;` -> isparsinti `JMP`  ir priskirti i enum value

==============================================================
Symbolic syntax: dest = comp ; jump
==============================================================
Binary syntax: 1 1 1 a c c c c c c d d d j j j
==============================================================

COMP        c c c c c c
-----------------------
0   |     | 1 0 1 0 1 0
1   |     | 1 1 1 1 1 1
-1  |     | 1 1 1 0 1 0
D   |     | 0 0 1 1 0 0
A   | M   | 1 1 0 0 0 0
!D  |     | 0 0 1 1 0 1
!A  | !M  | 1 1 0 0 0 1
-D  |     | 0 0 1 1 1 1
-A  | -M  | 1 1 0 0 1 1
D+1 |     | 0 1 1 1 1 1
A+1 | M+1 | 1 1 0 1 1 1
D-1 |     | 0 0 1 1 1 0
A-1 | M-1 | 1 1 0 0 1 0
D+A | D+M | 0 0 0 0 1 0
D-A | D-M | 0 1 0 0 1 1
A-D | M-D | 0 0 0 1 1 1
D&A | D&M | 0 0 0 0 0 0
D|A | D|M | 0 1 0 1 0 1
-----------------------
a=0   a=1

==============================================================

dest d d d effect: the value is stored in:
-----------------------------------------------------
null | 0 | 0 | 0 | the value is not stored
M    | 0 | 0 | 1 | RAM[A]
D    | 0 | 1 | 0 | D register
DM   | 0 | 1 | 1 | D register and RAM[A]
A    | 1 | 0 | 0 | A register
AM   | 1 | 0 | 1 | A register and RAM[A]
AD   | 1 | 1 | 0 | A register and D register
ADM  | 1 | 1 | 1 | A register, D register, and RAM[A]
-----------------------------------------------------

==============================================================

jump j j j effect:
--------------------------------------
null | 0 | 0 | 0 | no jump
JGT  | 0 | 0 | 1 | if comp > 0 jump
JEQ  | 0 | 1 | 0 | if comp = 0 jump
JGE  | 0 | 1 | 1 | if comp ≥ 0 jump
JLT  | 1 | 0 | 0 | if comp < 0 jump
JNE  | 1 | 0 | 1 | if comp ≠ 0 jump
JLE  | 1 | 1 | 0 | if comp ≤ 0 jump
JMP  | 1 | 1 | 1 | Unconditional jump
--------------------------------------

--------------------------------------------------
// LATER: saugoti abidvi komanadas i enuma. ir abudu enumai tures `fn toMachine() -> String`


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


