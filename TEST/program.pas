PROGRAM Sens_of_Universe_and_Everything;

VAR
x:Int64;

BEGIN
x:=25;
WHILE(TRUE) DO
        begin
          readln(x);
          IF x<>42 THEN
                    writeln(x)
          ELSE
                BREAK;
        end;
END.