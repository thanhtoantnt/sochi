# SolidiFI benchmark results


| Bug Type                      | Slither  | Mythril  | Confuzzius |
| :---------------------------- | :------: | :------: | :---------:|
| Re-entrancy (1343)            | 1380     | 105      | 231        |
| Timestamp Dependency (1381)   | 1149     | 76       | 620        |
| Unchecked Send (1266)         | -        | 2        | 0          |
| Unhandled Exceptions (1374)   | 826      | 26       | 429        |
| TOD (1336)                    | -        | -        | 0          |
| Integer Over/Underflow (1333) | -        | 43       | 0          |
| Use of tx.origin (1336)       | 1337     | 658      | -          |


