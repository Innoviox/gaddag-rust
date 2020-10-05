use crate::board::STATE;
use crate::game::Game;

pub fn main(rack: String) {
    let mut game = Game::default();

    let mut done = false;
    while !done {
        println!("input ready");
        let mut copy_state = STATE.clone();
        let mut bag: Vec<char> = vec![];
        for i in 0..15 {
            let mut row = String::new();
            std::io::stdin().read_line(&mut row).unwrap();
            for (idx, c) in row.trim().chars().enumerate() {
                if idx < 15 && c != '.' {
                    let c = c.to_uppercase().nth(0).unwrap();
                    copy_state[i][idx] = c;
                    bag.push(c);
                }
            }
        }
        game.set_board(copy_state);
        println!("{}", game.get_board());
        done = true;
    }

    game.get_player_mut(0).set_rack(rack.chars().collect());

    let (m1, sm1, _, _nmoves1) = game.do_move(1, false);

    println!(
        "{}",
        format!(
            "{:<7}/{:<3}: {:<12} +{:<03}/{:<03}",
            rack,
            m1.position.to_str(m1.direction),
            sm1,
            m1.score,
            game.get_player(0).score,
        )
    );
    println!("{}", game.get_board());
}

/*
...............
...............
...............
...............
...q...........
...it........v.
....roam....fit
....a.toy..tear
....n..halter.i
.jouk.cow..n.lo
.i..i.....ut.ax
.v..n.....peh.i
fe..g.gan..dead
ed...zap....wee
y.....bongoes.s
*/

/*
...............
............B..
............R..
...........GI..
..........MAN..
....AAL...OI...
.....DAG..AN...
......GOX..LIEU
.......RITZY...
...QUIRE..E....
..HIN.....D..C.
T.ES......SHARK
YAW..........AE
R............NE
O............EF
*/

/*
...............
...............
...............
...............
...............
.........L.....
.......FOALS...
......WAWS.ET..
......H....AH..
.....BI....MA..
.....ET....IT..
...DAYS....N...
...........G...
...............
...............
*/

/*
...............
...............
.JUG...........
..PIC......V..S
....HOP...ZA..M
....IRONED.R..A
........MEDICOS
...GOATEE..XU.H
............BE.
....DAL......N.
...NOMINEE...G.
..YAW...TROVER.
.KEEN........A.
.........WEB.F.
..........FYTTE
*/

/*
...............
......SPIRITED.
...........E...
...........L...
..........JOE..
..........OIL..
........WAE....
......NAOI.....
.....VANE......
....CIG........
.....E.A..FEDS.
.....SAGGIER.OW
......MA..HA.UH
.....VATU...OKA
.......E...XI.T
*/

/*
KAF.BOOMLET...J
.RELAY...N.Q..U
..N......G.U..T
..DEEM..RONIONS
....WAITERED
....EX...G
......PLIED
...CAHOOTS
....WAP
.STING
.HONS.....FAB
.I.....CAGEY
.V.....E..REZ
RADIATOR
.......T
*/

/*
............MAX
............OBI
...........QI
...........U
..HIJRA....E
TUM......V.A
H.......PIANS
AT.CONKERS.SPUN
WO.I..ANY...A
.F.V.LEG....R
BUTE........G
I.IT...DIDDLEY
G.C..ROOF...R
OE
TRENAILS
*/

/*
...W..........R
...A.....Q....O
...I.....U....B
...F...ALIENESS
.BASTION.T
GO......TE...C
.......WO..V.I
......HIND.IOTA
...U.FAN.EME.E
...R.ORG.XI..RE
...CRUDS..BAP
...H.......LI
..JIVED....EL
.MON..U..YORE
ZA.SANDY...T
*/

/*
...............
...............
......J...QAT..
......AG...X...
......MO..KI...
.......RIPES...
.......G..Y....
.......E.......
...............
...............
...............
...............
...............
...............
...............
*/

/*
hi.bitten
er.e.air
dele
o.ar
n.z
i.e.mug
s..win
trio..jut
..do.pomos
...flag.rif
....ads..cee
..tav...hern..c
.vela.koi..duly
..............a
..............n
*/

/*
.....hind...qua
....won.en.fa
..trod..valet
..e......tew
..c.er...ti
....xi...e..g
...d.map.r..l
ceder.logs..o
...ya.ado...z
....jus.brome
...ya......i
..aa.......la
..i....ego.ki
.nth.refines
.u.i
*/
