package rvs_pkg;
    typedef chandle rvs_context;
    typedef chandle rvs_error;

    typedef int unsigned uint32_t;
    typedef int unsigned rvs_handle;
    typedef int unsigned rvs_result;
    typedef int unsigned rvs_error_code;

    import "DPI-C" function rvs_context rvs_context_new();
    import "DPI-C" function void rvs_context_free(rvs_context ctxt);
    import "DPI-C" function void rvs_seed(rvs_context ctxt, u32_t seed);
    import "DPI-C" function void rvs_parse(rvs_context ctxt, string s, rvs_error error);
    import "DPI-C" function rvs_handle rvs_find(rvs_context ctxt, string name);
    import "DPI-C" function rvs_result rvs_next(rvs_context ctxt, rvs_handle handle);
    import "DPI-C" function rvs_result rvs_prev(rvs_context ctxt, rvs_handle handle);
    import "DPI-C" function bit rvs_done(rvs_context ctxt, rvs_handle handle);

    import "DPI-C" function rvs_error rvs_error_new();
    import "DPI-C" function void rvs_error_free(rvs_error error);
    import "DPI-C" function rvs_error_code rvs_error_code(rvs_error error);
    import "DPI-C" function string rvs_error_message(rvs_error error);

    /// Represents an Rvs variable
    ///
    /// Should not be created directly.  Should be created via Rvs::find().
    class Rv;
        local rvs_context m_ctxt;
        local rvs_handle m_handle;

        function new(rvs_context ctxt, rvs_handle handle);
            m_ctxt = ctxt;
            m_handle = handle;
        endfunction

        static function Rv create(rvs_context ctxt, rvs_handle handle);
            Rv rv = new(ctxt, handle);
            return rv;
        endfunction

        function rvs_result next();
            return rvs_next(m_ctxt, m_handle);
        endfunction

        function rvs_result prev();
            return rvs_prev(m_ctxt, m_handle);
        endfunction

        function bit done();
            return rvs_done(m_ctxt, m_handle);
        endfunction
    endclass

    /// Manages an Rvs context
    class Rvs;
        local rvs_context m_ctxt;
        local rvs_error m_error;

        local static Rvs m_inst;

        local function new();
            m_ctxt = rvs_context_new();
            m_error = rvs_error_new();
        endfunction

        local static function void init();
            if (!m_inst) m_inst = new();
        endfunction

        static function void parse(string s);
            init();

            rvs_parse(m_ctxt, s, m_error);

            if (rvs_error_code(m_error)) begin
                $fatal(1, "Could not parse: '%s'", rvs_error_message(m_error));
            end
        endfunction

        static function Rv find(string name);
            init();

            begin
                rvs_handle handle = rvs_find(m_ctxt, name, handle);

                if (!handle) begin
                    $fatal(1, "Variable '%s' not found in context", name);
                end

                return Rv::create(m_ctxt, handle);
            end
        endfunction

        static function bit exists(string name);
            init();

            begin
                rvs_handle handle = rvs_find(m_ctxt, name, handle);

                return handle > 0;
            end
        endfunction
    endclass
endpackage
