#pragma once
// clang-format off
#ifndef __cplusplus

#define z_loan(x) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_loan, \
        z_owned_bytes_reader_t : z_bytes_reader_loan, \
        z_owned_condvar_t : z_condvar_loan, \
        z_owned_config_t : z_config_loan, \
        z_owned_encoding_t : z_encoding_loan, \
        z_owned_hello_t : z_hello_loan, \
        z_owned_keyexpr_t : z_keyexpr_loan, \
        z_owned_publisher_t : z_publisher_loan, \
        z_owned_query_t : z_query_loan, \
        z_owned_sample_t : z_sample_loan, \
        z_owned_session_t : z_session_loan, \
        z_owned_slice_t : z_slice_loan, \
        z_owned_slice_map_t : z_slice_map_loan, \
        z_owned_str_array_t : z_str_array_loan, \
        z_owned_str_t : z_str_loan, \
        z_owned_subscriber_t : z_subscriber_loan, \
        z_view_keyexpr_t : z_view_keyexpr_loan, \
        z_view_slice_t : z_view_slice_loan, \
        z_view_str_t : z_view_str_loan, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_loan \
    )(&x)

#define z_loan_mut(x) \
    _Generic((x), \
        z_owned_bytes_reader_t : z_bytes_reader_loan_mut, \
        z_owned_condvar_t : z_condvar_loan_mut, \
        z_owned_config_t : z_config_loan_mut, \
        z_owned_mutex_t : z_mutex_loan_mut, \
        z_owned_slice_map_t : z_slice_map_loan_mut \
    )(&x)

#define z_drop(x) \
    _Generic((x),\
        z_owned_bytes_t * : z_bytes_drop,\
        z_owned_bytes_reader_t * : z_bytes_reader_drop,\
        z_owned_closure_hello_t * : z_closure_hello_drop,\
        z_owned_closure_owned_query_t * : z_closure_owned_query_drop,\
        z_owned_closure_query_t * : z_closure_query_drop,\
        z_owned_closure_reply_t * : z_closure_reply_drop,\
        z_owned_closure_sample_t * : z_closure_sample_drop,\
        z_owned_closure_zid_t * : z_closure_zid_drop,\
        z_owned_condvar_t * : z_condvar_drop,\
        z_owned_config_t * : z_config_drop,\
        z_owned_encoding_t * : z_encoding_drop,\
        z_owned_hello_t * : z_hello_drop,\
        z_owned_keyexpr_t * : z_keyexpr_drop,\
        z_owned_mutex_t * : z_mutex_drop,\
        z_owned_query_channel_closure_t * : z_query_channel_closure_drop,\
        z_owned_query_channel_t * : z_query_channel_drop,\
        z_owned_query_t * : z_query_drop,\
        z_owned_reply_channel_closure_t * : z_reply_channel_closure_drop,\
        z_owned_reply_channel_t * : z_reply_channel_drop,\
        z_owned_reply_t * : z_reply_drop,\
        z_owned_sample_t * : z_sample_drop,\
        z_owned_scouting_config_t * : z_scouting_config_drop,\
        z_owned_slice_t * : z_slice_drop,\
        z_owned_slice_map_t * : z_slice_map_drop,\
        z_owned_str_array_t * : z_str_array_drop,\
        z_owned_str_t * : z_str_drop,\
        zc_owned_shm_manager_t * : zc_shm_manager_drop,\
        zc_owned_shmbuf_t * : zc_shmbuf_drop,\
        zcu_owned_closure_matching_status_t * : zcu_closure_matching_status_drop \
    )(x)

#define z_null(x) \
    _Generic((x), \
        z_owned_bytes_t * : z_bytes_null, \
        z_owned_bytes_reader_t * : z_bytes_reader_null, \
        z_owned_condvar_t * : z_condvar_null, \
        z_owned_config_t * : z_config_null, \
        z_owned_encoding_t * : z_encoding_null, \
        z_owned_hello_t * : z_hello_null, \
        z_owned_keyexpr_t * : z_keyexpr_null, \
        z_owned_mutex_t * : z_mutex_null, \
        z_owned_publisher_t * : z_publisher_null, \
        z_owned_query_t * : z_query_null, \
        z_owned_queryable_t * : z_queryable_null, \
        z_owned_reply_t * : z_reply_null, \
        z_owned_sample_t * : z_sample_null, \
        z_owned_scouting_config_t * : z_scouting_config_null, \
        z_owned_session_t * : z_session_null, \
        z_owned_slice_map_t * : z_slice_map_null, \
        z_owned_slice_t * : z_slice_null, \
        z_owned_str_t * : z_str_null, \
        z_owned_subscriber_t * : z_subscriber_null, \
        z_owned_task_t * : z_task_null, \
        z_view_keyexpr_t * : z_view_keyexpr_null, \
        z_view_slice_t * : z_view_slice_null, \
        z_view_str_t * : z_view_str_null, \
        zc_owned_liveliness_token_t * : zc_liveliness_token_null, \
        ze_owned_publication_cache_t * : ze_publication_cache_null, \
        ze_owned_querying_subscriber_t * : ze_querying_subscriber_null \
    )(x)

#define z_check(x) \
    _Generic((x), \
        z_owned_bytes_t : z_bytes_check, \
        z_owned_bytes_reader_t : z_bytes_reader_check, \
        z_owned_condvar_t : z_condvar_check, \
        z_owned_config_t : z_config_check, \
        z_owned_encoding_t : z_encoding_check, \
        z_owned_hello_t : z_hello_check, \
        z_owned_keyexpr_t : z_keyexpr_check, \
        z_owned_mutex_t : z_mutex_check, \
        z_owned_slice_t : z_owned_slice_check, \
        z_owned_publisher_t : z_publisher_check, \
        z_owned_query_t : z_query_check, \
        z_owned_queryable_t : z_queryable_check, \
        z_owned_reply_t : z_reply_check, \
        z_owned_sample_t : z_sample_check, \
        z_owned_scouting_config_t : z_scouting_config_check, \
        z_owned_session_t : z_session_check, \
        z_owned_slice_map_t : z_slice_map_check, \
        z_owned_str_array_t : z_str_array_check, \
        z_owned_str_t : z_str_check, \
        z_owned_subscriber_t : z_subscriber_check, \
        z_owned_task_t : z_task_check, \
        z_view_keyexpr_t : z_view_keyexpr_check, \
        z_view_slice_t : z_view_slice_check, \
        zc_owned_liveliness_token_t : zc_liveliness_token_check, \
        zc_owned_shm_manager_t : zc_shm_manager_check, \
        zc_owned_shmbuf_t : zc_shmbuf_check, \
        ze_owned_publication_cache_t : ze_publication_cache_check, \
        ze_owned_querying_subscriber_t : ze_querying_subscriber_check \
    )(&x)

#define z_call(x, ...) \
     _Generic((x), \
        z_owned_closure_hello_t : z_closure_hello_call, \
        z_owned_closure_owned_query_t : z_closure_owned_query_call, \
        z_owned_closure_query_t : z_closure_query_call, \
        z_owned_closure_reply_t : z_closure_reply_call, \
        z_owned_closure_sample_t : z_closure_sample_call, \
        z_owned_closure_zid_t : z_closure_zid_call, \
        z_owned_query_channel_closure_t : z_query_channel_closure_call, \
        z_owned_reply_channel_closure_t : z_reply_channel_closure_call, \
        zcu_owned_closure_matching_status_t : zcu_closure_matching_status_call \
    )(&x, __VA_ARGS__)

#define z_closure(x, callback, dropper, ctx) \
     _Generic((x), \
        z_owned_closure_hello_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        z_owned_closure_owned_query_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        z_owned_closure_query_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        z_owned_closure_reply_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        z_owned_closure_sample_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        z_owned_closure_zid_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        z_owned_query_channel_closure_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        z_owned_reply_channel_closure_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; }, \
        zcu_owned_closure_matching_status_t * : { x->context = (void*)ctx; x->call = callback; x->drop = droper; } \
    )
#else  // #ifndef __cplusplus



inline const z_loaned_bytes_t* z_loan(const z_owned_bytes_t* payload) { return z_bytes_loan(payload); };
inline const z_loaned_bytes_reader_t* z_loan(const z_owned_bytes_reader_t* reader) { return z_bytes_reader_loan(reader); };
inline const z_loaned_condvar_t* z_loan(const z_owned_condvar_t* this_) { return z_condvar_loan(this_); };
inline const z_loaned_config_t* z_loan(const z_owned_config_t* this_) { return z_config_loan(this_); };
inline const z_loaned_encoding_t* z_loan(const z_owned_encoding_t* encoding) { return z_encoding_loan(encoding); };
inline const z_hello_t* z_loan(const z_owned_hello_t* hello) { return z_hello_loan(hello); };
inline const z_loaned_keyexpr_t* z_loan(const z_owned_keyexpr_t* key_expr) { return z_keyexpr_loan(key_expr); };
inline const z_loaned_publisher_t* z_loan(const z_owned_publisher_t* this_) { return z_publisher_loan(this_); };
inline const z_loaned_query_t* z_loan(const z_owned_query_t* this_) { return z_query_loan(this_); };
inline const z_loaned_sample_t* z_loan(const z_owned_sample_t* sample) { return z_sample_loan(sample); };
inline const z_loaned_session_t* z_loan(const z_owned_session_t* this_) { return z_session_loan(this_); };
inline const z_loaned_slice_t* z_loan(const z_owned_slice_t* this_) { return z_slice_loan(this_); };
inline const z_loaned_slice_map_t* z_loan(const z_owned_slice_map_t* this_) { return z_slice_map_loan(this_); };
inline const z_str_array_t* z_loan(const z_owned_str_array_t* strs) { return z_str_array_loan(strs); };
inline const z_loaned_str_t* z_loan(const z_owned_str_t* this_) { return z_str_loan(this_); };
inline const z_loaned_subscriber_t* z_loan(const z_owned_subscriber_t* this_) { return z_subscriber_loan(this_); };
inline const z_loaned_keyexpr_t* z_loan(const z_view_keyexpr_t* key_expr) { return z_view_keyexpr_loan(key_expr); };
inline const z_loaned_slice_t* z_loan(const z_view_slice_t* this_) { return z_view_slice_loan(this_); };
inline const z_loaned_str_t* z_loan(const z_view_str_t* this_) { return z_view_str_loan(this_); };
inline const ze_loaned_querying_subscriber_t* z_loan(const ze_owned_querying_subscriber_t* this_) { return ze_querying_subscriber_loan(this_); };


inline z_loaned_bytes_reader_t* z_loan_mut( z_owned_bytes_reader_t& reader) { return z_bytes_reader_loan_mut(&reader); };
inline z_loaned_condvar_t* z_loan_mut( z_owned_condvar_t& this_) { return z_condvar_loan_mut(&this_); };
inline z_loaned_config_t* z_loan_mut( z_owned_config_t& this_) { return z_config_loan_mut(&this_); };
inline z_loaned_mutex_t* z_loan_mut( z_owned_mutex_t& this_) { return z_mutex_loan_mut(&this_); };
inline z_loaned_slice_map_t* z_loan_mut( z_owned_slice_map_t& this_) { return z_slice_map_loan_mut(&this_); };


inline void z_drop(const z_owned_bytes_t* this_) { return z_bytes_drop(this_); };
inline void z_drop(const z_owned_bytes_reader_t* this_) { return z_bytes_reader_drop(this_); };
inline void z_drop(const z_owned_closure_hello_t* closure) { return z_closure_hello_drop(closure); };
inline void z_drop(const z_owned_closure_owned_query_t* closure) { return z_closure_owned_query_drop(closure); };
inline void z_drop(const z_owned_closure_query_t* closure) { return z_closure_query_drop(closure); };
inline void z_drop(const z_owned_closure_reply_t* closure) { return z_closure_reply_drop(closure); };
inline void z_drop(const z_owned_closure_sample_t* closure) { return z_closure_sample_drop(closure); };
inline void z_drop(const z_owned_closure_zid_t* closure) { return z_closure_zid_drop(closure); };
inline void z_drop(const z_owned_condvar_t* this_) { return z_condvar_drop(this_); };
inline void z_drop(const z_owned_config_t* config) { return z_config_drop(config); };
inline void z_drop(const z_owned_encoding_t* encoding) { return z_encoding_drop(encoding); };
inline void z_drop(const z_owned_hello_t* hello) { return z_hello_drop(hello); };
inline void z_drop(const z_owned_keyexpr_t* keyexpr) { return z_keyexpr_drop(keyexpr); };
inline void z_drop(const z_owned_mutex_t* this_) { return z_mutex_drop(this_); };
inline void z_drop(const z_owned_query_channel_closure_t* closure) { return z_query_channel_closure_drop(closure); };
inline void z_drop(const z_owned_query_channel_t* channel) { return z_query_channel_drop(channel); };
inline void z_drop(const z_owned_query_t* this_) { return z_query_drop(this_); };
inline void z_drop(const z_owned_reply_channel_closure_t* closure) { return z_reply_channel_closure_drop(closure); };
inline void z_drop(const z_owned_reply_channel_t* channel) { return z_reply_channel_drop(channel); };
inline void z_drop(const z_owned_reply_t* this_) { return z_reply_drop(this_); };
inline void z_drop(const z_owned_sample_t* sample) { return z_sample_drop(sample); };
inline void z_drop(const z_owned_scouting_config_t* config) { return z_scouting_config_drop(config); };
inline void z_drop(const z_owned_slice_t* this_) { return z_slice_drop(this_); };
inline void z_drop(const z_owned_slice_map_t* this_) { return z_slice_map_drop(this_); };
inline void z_drop(const z_owned_str_array_t* strs) { return z_str_array_drop(strs); };
inline void z_drop(const z_owned_str_t* this_) { return z_str_drop(this_); };
inline void z_drop(const zc_owned_shm_manager_t* manager) { return zc_shm_manager_drop(manager); };
inline void z_drop(const zc_owned_shmbuf_t* buf) { return zc_shmbuf_drop(buf); };
inline void z_drop(const zcu_owned_closure_matching_status_t* closure) { return zcu_closure_matching_status_drop(closure); };


inline void z_null( z_owned_bytes_t* this_) { return z_bytes_null(this_); };
inline void z_null( z_owned_bytes_reader_t* this_) { return z_bytes_reader_null(this_); };
inline void z_null( z_owned_condvar_t* this_) { return z_condvar_null(this_); };
inline void z_null( z_owned_config_t* this_) { return z_config_null(this_); };
inline void z_null( z_owned_encoding_t* encoding) { return z_encoding_null(encoding); };
inline void z_null( z_owned_hello_t* this_) { return z_hello_null(this_); };
inline void z_null( z_owned_keyexpr_t* this_) { return z_keyexpr_null(this_); };
inline void z_null( z_owned_mutex_t* this_) { return z_mutex_null(this_); };
inline void z_null( z_owned_publisher_t* this_) { return z_publisher_null(this_); };
inline void z_null( z_owned_query_t* this_) { return z_query_null(this_); };
inline void z_null( z_owned_queryable_t* this_) { return z_queryable_null(this_); };
inline void z_null( z_owned_reply_t* this_) { return z_reply_null(this_); };
inline void z_null( z_owned_sample_t* sample) { return z_sample_null(sample); };
inline void z_null( z_owned_scouting_config_t* this_) { return z_scouting_config_null(this_); };
inline void z_null( z_owned_session_t* this_) { return z_session_null(this_); };
inline void z_null( z_owned_slice_map_t* this_) { return z_slice_map_null(this_); };
inline void z_null( z_owned_slice_t* this_) { return z_slice_null(this_); };
inline void z_null( z_owned_str_t* this_) { return z_str_null(this_); };
inline void z_null( z_owned_subscriber_t* this_) { return z_subscriber_null(this_); };
inline void z_null( z_owned_task_t* this_) { return z_task_null(this_); };
inline void z_null( z_view_keyexpr_t* this_) { return z_view_keyexpr_null(this_); };
inline void z_null( z_view_slice_t* this_) { return z_view_slice_null(this_); };
inline void z_null( z_view_str_t* this_) { return z_view_str_null(this_); };
inline void z_null( zc_owned_liveliness_token_t* this_) { return zc_liveliness_token_null(this_); };
inline void z_null( ze_owned_publication_cache_t* this_) { return ze_publication_cache_null(this_); };
inline void z_null( ze_owned_querying_subscriber_t* this_) { return ze_querying_subscriber_null(this_); };


inline bool z_check(const z_owned_bytes_t& payload) { return z_bytes_check(&payload); };
inline bool z_check(const z_owned_bytes_reader_t& this_) { return z_bytes_reader_check(&this_); };
inline bool z_check(const z_owned_condvar_t& this_) { return z_condvar_check(&this_); };
inline bool z_check(const z_owned_config_t& config) { return z_config_check(&config); };
inline bool z_check(const z_owned_encoding_t& encoding) { return z_encoding_check(&encoding); };
inline bool z_check(const z_owned_hello_t& hello) { return z_hello_check(&hello); };
inline bool z_check(const z_owned_keyexpr_t& keyexpr) { return z_keyexpr_check(&keyexpr); };
inline bool z_check(const z_owned_mutex_t& this_) { return z_mutex_check(&this_); };
inline bool z_check(const z_owned_slice_t& this_) { return z_owned_slice_check(&this_); };
inline bool z_check(const z_owned_publisher_t& this_) { return z_publisher_check(&this_); };
inline bool z_check(const z_owned_query_t& query) { return z_query_check(&query); };
inline bool z_check(const z_owned_queryable_t& qable) { return z_queryable_check(&qable); };
inline bool z_check(const z_owned_reply_t& this_) { return z_reply_check(&this_); };
inline bool z_check(const z_owned_sample_t& sample) { return z_sample_check(&sample); };
inline bool z_check(const z_owned_scouting_config_t& config) { return z_scouting_config_check(&config); };
inline bool z_check(const z_owned_session_t& this_) { return z_session_check(&this_); };
inline bool z_check(const z_owned_slice_map_t& map) { return z_slice_map_check(&map); };
inline bool z_check(const z_owned_str_array_t& strs) { return z_str_array_check(&strs); };
inline bool z_check(const z_owned_str_t& this_) { return z_str_check(&this_); };
inline bool z_check(const z_owned_subscriber_t& subscriber) { return z_subscriber_check(&subscriber); };
inline bool z_check(const z_owned_task_t& this_) { return z_task_check(&this_); };
inline bool z_check(const z_view_keyexpr_t& keyexpr) { return z_view_keyexpr_check(&keyexpr); };
inline bool z_check(const z_view_slice_t& this_) { return z_view_slice_check(&this_); };
inline bool z_check(const zc_owned_liveliness_token_t& this_) { return zc_liveliness_token_check(&this_); };
inline bool z_check(const zc_owned_shm_manager_t& manager) { return zc_shm_manager_check(&manager); };
inline bool z_check(const zc_owned_shmbuf_t& buf) { return zc_shmbuf_check(&buf); };
inline bool z_check(const ze_owned_publication_cache_t& this_) { return ze_publication_cache_check(&this_); };
inline bool z_check(const ze_owned_querying_subscriber_t& this_) { return ze_querying_subscriber_check(&this_); };


inline void z_call(const z_owned_closure_hello_t& closure,  z_owned_hello_t* hello) {
    return z_closure_hello_call(&closure, hello); 
};
inline void z_call(const z_owned_closure_owned_query_t& closure,  z_owned_query_t* query) {
    return z_closure_owned_query_call(&closure, query); 
};
inline void z_call(const z_owned_closure_query_t& closure, const z_loaned_query_t* query) {
    return z_closure_query_call(&closure, query); 
};
inline void z_call(const z_owned_closure_reply_t& closure, const z_loaned_reply_t* reply) {
    return z_closure_reply_call(&closure, reply); 
};
inline void z_call(const z_owned_closure_sample_t& closure, const z_loaned_sample_t* sample) {
    return z_closure_sample_call(&closure, sample); 
};
inline void z_call(const z_owned_closure_zid_t& closure, const z_id_t* sample) {
    return z_closure_zid_call(&closure, sample); 
};
inline bool z_call(const z_owned_query_channel_closure_t& closure,  z_owned_query_t* query) {
    return z_query_channel_closure_call(&closure, query); 
};
inline bool z_call(const z_owned_reply_channel_closure_t& closure,  z_owned_reply_t* reply) {
    return z_reply_channel_closure_call(&closure, reply); 
};
inline void z_call(const zcu_owned_closure_matching_status_t& closure, const zcu_matching_status_t* sample) {
    return zcu_closure_matching_status_call(&closure, sample); 
};


inline void z_closure(
    z_owned_closure_hello_t* closure,
    void (*call)( z_owned_hello_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_owned_query_t* closure,
    void (*call)( z_owned_query_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_query_t* closure,
    void (*call)(const z_loaned_query_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_reply_t* closure,
    void (*call)(const z_loaned_reply_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_sample_t* closure,
    void (*call)(const z_loaned_sample_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_closure_zid_t* closure,
    void (*call)(const z_id_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_query_channel_closure_t* closure,
    bool (*call)( z_owned_query_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    z_owned_reply_channel_closure_t* closure,
    bool (*call)( z_owned_reply_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
inline void z_closure(
    zcu_owned_closure_matching_status_t* closure,
    void (*call)(const zcu_matching_status_t*, void*),
    void (*drop)(void*),
    void *context) {
    closure->context = context;
    closure->drop = drop;
    closure->call = call;
};
#endif  // #ifndef __cplusplus