pub mod module_c;
mod module_d;
use self::module_c::func_c; // module_cがmodule_bの中にあるから、selfを使える。
