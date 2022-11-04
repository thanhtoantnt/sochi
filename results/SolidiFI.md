# SolidiFI benchmark results


| Bug Type                      | Slither     | Mythril  | Confuzzius |
| :---------------------------- | :---------: | :------: | :---------:|
| Re-entrancy (1343)            | 1343 (1380) | 105      | 231        |
| Timestamp Dependency (1381)   | 1134 (1149) | 76       | 620        |
| Unchecked Send (1266)         | -           | 2        | 29         |
| Unhandled Exceptions (1374)   | 828 (828)   | 26       | 429        |
| TOD (1336)                    | -           | -        | 4          |
| Integer Over/Underflow (1333) | -           | 43       | 750        |
| Use of tx.origin (1336)       | (1337)      | 658      | -          |


