# TZ_auction

```
NFT Auction 
(имплементация должна исходить из того что смарт-контракты реактивны 
т.е. не делают ничего без действия с внешней стороны, 
к примеру по истечению времени аукцион сам не завершится если кто-то не вызовет метод finalize итп)

У пользователей есть баланс в некоторой валюте. 
Баланс представлен как целочисленное представления числа с плавающей точкой с точностью до 8 знака e.g. 123456789u128 is 1.23456789 in tokens

Владелец лота (NFT) может выставить его на продажу
аукцион длится N-секунд

-аукцион позволяет пользователю сделать ставку на Лот,
-забрать ставку если его ставка не наибольшая
-повысить ставку если его ставка не наибольшая
-забрать лот если аукцион завершился и его ставка наибольшая, в этом случае деньги переходят на баланс аукциона, а Лот передаётся победителю аукциона

также лот может иметь цену выкупа (при достижении это цены аукцион завершается)
также лот может иметь скрытую reserve price (цена ниже которой лот не будет продан, даже если есть победитель)
```
