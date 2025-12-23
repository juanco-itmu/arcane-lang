// Dit is 'n Arcane program
// Bereken die som van 1 tot 10

stel x = 10
stel som = 0

terwyl (x > 0) {
    stel som = som + x
    stel x = x - 1
}

druk(som)

// Toets if/else
stel getal = 42
as (getal > 50) {
    druk(1)
} anders {
    druk(0)
}

// Toets booleans
stel aktief = waar
as (aktief && getal == 42) {
    druk(waar)
}

druk(vals)
