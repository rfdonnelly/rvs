# Rvs SystemVerilog Bindings

## Usage Example

```SystemVerilog
import rvs_pkg::Rvs;
import rvs_pkg::Rv;

Rvs::parse("a = 0;");
Rv a = Rvs::find("a");

assert (a.next() == 0) begin
    $info("a.next() == 0");
end else begin
    $fatal(1, "a.next() != 0");
end
```
