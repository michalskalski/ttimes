ttimes - Display WEC race times in terminal
=========

Description
---------

```
Simple script to display times of WEC race.

USAGE:
    ttimes [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --categories <categories>    Takes comma separated list of WEC car categories to display
    -u, --url <url>                  Url for race json data
```

Developed during 87 Edition des 24 Heures du Mans.

Example
---------

```
./ttimes -c LMGTEPro
87º Edition des 24 Heures du Mans - Status: green -  Elapsed time: 24:00:04
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| Pos | Num | Cat      | PiC | Driver/Team                            | State | Laps | Lap time | Int      | PIT |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 20  | 51  | LMGTEPro | 1   | PIER GUIDI A./AF Corse                 | Run   | 341  | 3:56.490 |          | 25  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 21  | 91  | LMGTEPro | 2   | BRUNI G./Porsche GT Team               | Run   | 341  | 3:52.732 | 54.045   | 23  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 22  | 93  | LMGTEPro | 3   | BAMBER E./Porsche GT Team              | Run   | 341  | 3:53.275 | 1:13.114 | 23  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 23  | 68  | LMGTEPro | 4   | HAND J./Ford Chip Ganassi Team USA     | Run   | 341  | 3:55.432 | 2:15.332 | 23  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 24  | 67  | LMGTEPro | 5   | TINCKNELL H./Ford Chip Ganassi Team UK | Run   | 341  | 3:53.382 | 3:24.296 | 24  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 25  | 69  | LMGTEPro | 6   | DIXON S./Ford Chip Ganassi Team USA    | Run   | 340  | 3:53.906 | 1 Lap    | 26  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 26  | 66  | LMGTEPro | 7   | PLA O./Ford Chip Ganassi Team UK       | Run   | 339  | 4:01.242 | 2 Laps   | 24  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 28  | 94  | LMGTEPro | 8   | JAMINET M./Porsche GT Team             | Run   | 338  | 3:52.624 | 3 Laps   | 24  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 29  | 63  | LMGTEPro | 9   | GARCIA A./Corvette Racing              | Run   | 336  | 3:56.778 | 5 Laps   | 25  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 30  | 92  | LMGTEPro | 10  | ESTRE K./Porsche GT Team               | Run   | 336  | 3:55.474 | 2:04.928 | 24  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 31  | 82  | LMGTEPro | 11  | KROHN J./BMW Team MTEK                 | Run   | 334  | 3:55.980 | 7 Laps   | 25  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 43  | 89  | LMGTEPro | 12  | GOUNON J./Risi Competizione            | Run   | 328  | 3:56.019 | 13 Laps  | 25  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 46  | 97  | LMGTEPro | 13  | ADAM J./Aston Martin Racing            | Run   | 324  | 3:57.059 | 17 Laps  | 25  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 49  | 81  | LMGTEPro | 14  | ENG P./BMW Team MTEK                   | Run   | 308  | 3:55.916 | 33 Laps  | 24  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 56  | 71  | LMGTEPro | 15  | MOLINA M./AF Corse                     | In    | 140  | 4:41.910 | 201 Laps | 11  |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 57  | 95  | LMGTEPro | 16  | SØRENSEN M./Aston Martin Racing        | Stop  | 132  | 4:26.994 | 209 Laps | 9   |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
| 59  | 64  | LMGTEPro | 17  | FÄSSLER M./Corvette Racing             | Stop  | 82   | 3:55.433 | 259 Laps | 5   |
+-----+-----+----------+-----+----------------------------------------+-------+------+----------+----------+-----+
```
