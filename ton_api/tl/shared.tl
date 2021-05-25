int ? = Int;
long ? = Long;
double ? = Double;
string ? = String;
object ? = Object;
function ? = Function;
bytes data:string = Bytes;
true = True;
boolTrue = Bool;
boolFalse = Bool;


vector {t:Type} # [ t ] = Vector t;

int128 4*[ int ] = Int128;
int256 8*[ int ] = Int256;

tcp.pong random_id:long = tcp.Pong;
adnl.pong value:long = adnl.Pong;

---functions---

tcp.ping random_id:long = tcp.Pong;
adnl.ping value:long = adnl.Pong;
