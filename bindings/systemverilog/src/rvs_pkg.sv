package rvs_pkg;
    typedef chandle rvs_context_t;
    typedef int unsigned uint32_t;
    typedef int unsigned rvs_handle_t;
    typedef int unsigned rvs_result_t;
    typedef int unsigned rvs_status_raw_t;

    virtual class rvs_status_t;
        typedef enum rvs_status_raw_t {
            Success = 0,
            NotFound = 1,
            ParseError = 2
        } e;
    endclass

    import "DPI-C" function rvs_context_t rvs_context_new();
    import "DPI-C" function void rvs_context_free(rvs_context_t ctxt);
    import "DPI-C" function void rvs_seed(rvs_context_t ctxt, u32_t seed);
    import "DPI-C" function status_t rvs_parse(rvs_context_t ctxt, string s);
    import "DPI-C" function status_t rvs_find(rvs_context_t ctxt, string name, output handle_t handle);
    import "DPI-C" function status_t rvs_next(rvs_context_t ctxt, handle_t handle, output result_t result);
    import "DPI-C" function status_t rvs_prev(rvs_context_t ctxt, handle_t handle, output result_t result);
    import "DPI-C" function status_t rvs_done(rvs_context_t ctxt, handle_t handle, output bit result);

    /// Represents an Rvs variable
    ///
    /// Should not be created directly.  Should be created via Rvs::find().
    class Rv;
        local rvs_context_t m_ctxt;
        local rvs_handle_t m_handle;

        function new(rvs_context_t ctxt, rvs_handle_t handle);
            m_ctxt = ctxt;
            m_handle = handle;
        endfunction

        static function Rv create(rvs_context_t ctxt, rvs_handle_t handle);
            Rv rv = new(ctxt, handle);
            return rv;
        endfunction

        function rvs_result_t next();
            rvs_result_t result;

            // NOTE: Status not checked for errors.  Will not generate errors
            // with good context and good handle.  Creation of Rv via Rvs
            // ensures both.
            rvs_next(m_ctxt, m_handle, result);

            return result;
        endfunction

        function rvs_result_t prev();
            rvs_result_t result;

            // NOTE: Status not checked for errors.  Will not generate errors
            // with good context and good handle.  Creation of Rv via Rvs
            // ensures both.
            rvs_prev(m_ctxt, m_handle, result);

            return result;
        endfunction

        function bit done();
            bit result;

            // NOTE: Status not checked for errors.  Will not generate errors
            // with good context and good handle.  Creation of Rv via Rvs
            // ensures both.
            rvs_done(m_ctxt, m_handle, bit);

            return result;
        endfunction
    endclass

    /// Manages an Rvs context
    class Rvs;
        local rvs_context_t m_ctxt;

        local static Rvs m_inst;

        local function new();
            m_ctxt = rvs_context_new();
        endfunction

        local static function void init();
            if (!m_inst) m_inst = new();
        endfunction

        static function void parse(string s);
            init();

            begin
                rvs_status_t::e status = rvs_status_t::e'(rvs_parse(m_ctxt, s));

                if (status == rvs_status_t::ParseError) begin
                    $fatal(1, "Could not parse: '%s'", s);
                end
            end
        endfunction

        static function Rv find(string name);
            init();

            begin
                rvs_handle_t handle;
                rvs_status_t::e status = rvs_status_t::e'(rvs_find(m_ctxt, name, handle));

                if (status == rvs_status_t::NotFound) begin
                    $fatal(1, "Variable '%s' not found in context", name);
                end

                return Rv::create(m_ctxt, handle);
            end
        endfunction

        static function bit exists(string name);
            init();

            begin
                rvs_handle_t handle;
                rvs_status_t::e status = rvs_status_t::e'(rvs_find(m_ctxt, name, handle));

                return status != rvs_status_t::NotFound;
            end
        endfunction
    endclass
endpackage
