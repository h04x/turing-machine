#Turing machine
Turing machine on rust. Based on HashMap's
##Turing machine programming guide
Simple instruction  looks like this  
```
# this is 
# comment

q01->q01R                 # this is comment too
Ident19->Ident29R         # Ident1 matched state, 9 matched symbol
Id\-en\>t19->Ident\\2\#9R # escape control symbols 
```
parse rule

<current state><current symbol>-><final state><final symbol><tape motion> 

here
match and chage states - any string (eg q0, q1, q123, anyName)
match and change symbols - any symbol
move cursor - L, R or N, move left, right and do not move respectively

program is a seqenspe of rules, rule per line. 
match state of first rule is a initial state.
change state without match state stop programю