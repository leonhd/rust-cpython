use libc::c_int;

pub const Py_mp_ass_subscript: c_int = 3;
pub const Py_mp_length: c_int = 4;
pub const Py_mp_subscript: c_int = 5;
pub const Py_nb_absolute: c_int = 6;
pub const Py_nb_add: c_int = 7;
pub const Py_nb_and: c_int = 8;
pub const Py_nb_bool: c_int = 9;
pub const Py_nb_divmod: c_int = 10;
pub const Py_nb_float: c_int = 11;
pub const Py_nb_floor_divide: c_int = 12;
pub const Py_nb_index: c_int = 13;
pub const Py_nb_inplace_add: c_int = 14;
pub const Py_nb_inplace_and: c_int = 15;
pub const Py_nb_inplace_floor_divide: c_int = 16;
pub const Py_nb_inplace_lshift: c_int = 17;
pub const Py_nb_inplace_multiply: c_int = 18;
pub const Py_nb_inplace_or: c_int = 19;
pub const Py_nb_inplace_power: c_int = 20;
pub const Py_nb_inplace_remainder: c_int = 21;
pub const Py_nb_inplace_rshift: c_int = 22;
pub const Py_nb_inplace_subtract: c_int = 23;
pub const Py_nb_inplace_true_divide: c_int = 24;
pub const Py_nb_inplace_xor: c_int = 25;
pub const Py_nb_int: c_int = 26;
pub const Py_nb_invert: c_int = 27;
pub const Py_nb_lshift: c_int = 28;
pub const Py_nb_multiply: c_int = 29;
pub const Py_nb_negative: c_int = 30;
pub const Py_nb_or: c_int = 31;
pub const Py_nb_positive: c_int = 32;
pub const Py_nb_power: c_int = 33;
pub const Py_nb_remainder: c_int = 34;
pub const Py_nb_rshift: c_int = 35;
pub const Py_nb_subtract: c_int = 36;
pub const Py_nb_true_divide: c_int = 37;
pub const Py_nb_xor: c_int = 38;
pub const Py_sq_ass_item: c_int = 39;
pub const Py_sq_concat: c_int = 40;
pub const Py_sq_contains: c_int = 41;
pub const Py_sq_inplace_concat: c_int = 42;
pub const Py_sq_inplace_repeat: c_int = 43;
pub const Py_sq_item: c_int = 44;
pub const Py_sq_length: c_int = 45;
pub const Py_sq_repeat: c_int = 46;
pub const Py_tp_alloc: c_int = 47;
pub const Py_tp_base: c_int = 48;
pub const Py_tp_bases: c_int = 49;
pub const Py_tp_call: c_int = 50;
pub const Py_tp_clear: c_int = 51;
pub const Py_tp_dealloc: c_int = 52;
pub const Py_tp_del: c_int = 53;
pub const Py_tp_descr_get: c_int = 54;
pub const Py_tp_descr_set: c_int = 55;
pub const Py_tp_doc: c_int = 56;
pub const Py_tp_getattr: c_int = 57;
pub const Py_tp_getattro: c_int = 58;
pub const Py_tp_hash: c_int = 59;
pub const Py_tp_init: c_int = 60;
pub const Py_tp_is_gc: c_int = 61;
pub const Py_tp_iter: c_int = 62;
pub const Py_tp_iternext: c_int = 63;
pub const Py_tp_methods: c_int = 64;
pub const Py_tp_new: c_int = 65;
pub const Py_tp_repr: c_int = 66;
pub const Py_tp_richcompare: c_int = 67;
pub const Py_tp_setattr: c_int = 68;
pub const Py_tp_setattro: c_int = 69;
pub const Py_tp_str: c_int = 70;
pub const Py_tp_traverse: c_int = 71;
pub const Py_tp_members: c_int = 72;
pub const Py_tp_getset: c_int = 73;
pub const Py_tp_free: c_int = 74;
#[cfg(Py_3_5)]
pub const Py_nb_matrix_multiply: c_int = 75;
#[cfg(Py_3_5)]
pub const Py_nb_inplace_matrix_multiply: c_int = 76;
#[cfg(Py_3_5)]
pub const Py_am_await: c_int = 77;
#[cfg(Py_3_5)]
pub const Py_am_aiter: c_int = 78;
#[cfg(Py_3_5)]
pub const Py_am_anext: c_int = 79;
#[cfg(Py_3_5)]
pub const Py_tp_finalize: c_int = 80;
#[cfg(Py_3_10)]
pub const Py_am_send: c_int = 81;
