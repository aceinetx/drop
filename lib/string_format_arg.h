#ifdef sv_fmt
#undef sv_fmt
#else
#define sv_fmt "%.*s"
#endif

#ifdef sv_arg
#undef sv_arg
#else
#define sv_arg(str) (drop::s32)(str).len, (str).ptr
#endif
