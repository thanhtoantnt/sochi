# ISSTA'21` benchmark results

## CV benchmarks

| Bug Type                      | Slither  | Mythril  | Confuzzius |
| :---------------------------- | :------: | :------: | :---------:|
| Re-entrancy                   | 78       | 12       | 13         |
| Timestamp Dependency          | 21       | 3        | 17         |
| Unchecked Send                | -        | 5        | 6          |
| Unhandled Exceptions          | 6        | 10       | 90         |
| TOD                           | -        | -        | 26         |
| Integer Over/Underflow        | -        | 15       | 162        |
| Use of tx.origin              | 1        | 1        | -          |


## MI benchmarks

| Bug Type                      | Slither  | Mythril  | Confuzzius |
| :---------------------------- | :------: | :------: | :---------:|
| Re-entrancy                   | 1380     | 29       | 231        |
| Timestamp Dependency          | 1149     | 76       | 621        |
| Unchecked Send                | -        | 2        | 36         |
| Unhandled Exceptions          | 826      | 30       | 417        |
| TOD                           | -        | -        |  3         |
| Integer Over/Underflow        | -        | 43       | 765        |
| Use of tx.origin              | 1337     | 121      | 16         |


