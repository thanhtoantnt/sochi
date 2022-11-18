# SolidiFI benchmark results


| Bug Type                      | Slither     | Mythril  | Confuzzius |
| :---------------------------- | :---------: | :------: | :---------:|
| Re-entrancy (1343)            | 0 + 1343 (1380) | 1238 + 105      | 1112 + 231        |
| Timestamp Dependency (1381)   | 247 + 1134 (1149) | 1305 + 76       | 761 + 620        |
| Unchecked Send (1266)         | -           | 1264 + 2        | 1237 + 29         |
| Unhandled Exceptions (1374)   | 546 + 828 (828)   | 1348 + 26       | 429 + 945        |
| TOD (1336)                    | -           | -        | 1332 + 4          |
| Integer Over/Underflow (1333) | -           | 1290 + 43       | 583 + 750        |
| Use of tx.origin (1336)       | 68 + 1268 (1337) | 678 + 658      | -          |


