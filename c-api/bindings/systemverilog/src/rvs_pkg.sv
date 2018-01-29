package rvs_pkg;
    typedef chandle rvs_error;
    typedef chandle rvs_context;
    typedef chandle rvs_model;

    typedef int unsigned uint32_t;
    typedef int unsigned rvs_handle;
    typedef int unsigned rvs_result;
    typedef int unsigned rvs_error_code;

    import "DPI-C" function rvs_context rvs_context_new(string search_path, uint32_t seed, rvs_error error);
    import "DPI-C" function void rvs_context_free(rvs_context ctxt);
    import "DPI-C" function void rvs_parse(rvs_context ctxt, string s, rvs_error error);

    import "DPI-C" function rvs_model rvs_model_new();
    import "DPI-C" function void rvs_transform(rvs_context ctxt, rvs_model model, rvs_error error);
    import "DPI-C" function void rvs_model_free(rvs_model model);

    import "DPI-C" function rvs_handle rvs_get(rvs_model model, string name);
    import "DPI-C" function rvs_result rvs_next(rvs_model model, rvs_handle handle);
    import "DPI-C" function rvs_result rvs_prev(rvs_model model, rvs_handle handle);
    import "DPI-C" function bit rvs_done(rvs_model model, rvs_handle handle);
    import "DPI-C" function void rvs_write_definitions(rvs_model model, string name, rvs_error error);

    import "DPI-C" function rvs_error rvs_error_new();
    import "DPI-C" function bit rvs_error_test(rvs_error error);
    import "DPI-C" function string rvs_error_message(rvs_error error);
    import "DPI-C" function void rvs_error_free(rvs_error error);

    // Class: Rvs
    //
    // Static class for managing Rvs.
    //
    // Example usage:
    //
    //  initial begin
    //      Rvs::initialize(
    //          .search_path(Rvs::search_path_from_plusargs()),
    //          .seed(Rvs::seed_from_plusargs())
    //      );
    //      Rvs::parse("top.rvs");
    //      Rvs::parse_from_plusargs();
    //      Rvs::transform();
    //      Rvs::write_definitions("final.rvs");
    //  end
    //
    //  // After transform
    //  begin
    //      Rv a = new("a");
    //      $display(a.next());
    //  end
    //
    //  // At end-of-test
    //  Rvs::free();
    virtual class Rvs;
        static local rvs_error error;
        static local rvs_context ctxt;
        static local rvs_model model;

        // Function: initialize
        //
        // Initialize Rvs with a search path and seed.
        //
        // Example:
        //
        //  Rvs::initialize(
        //      .search_path(Rvs::search_path_from_plusargs()),
        //      .seed(Rvs::seed_from_plusargs())
        //  );
        static function void initialize(string search_path, uint32_t seed);
            error = rvs_error_new();
            model = rvs_model_new();
            ctxt = rvs_context_new(search_path, seed, error);
            handle_error();
        endfunction

        static function void free();
            if (ctxt) rvs_context_free(ctxt);
            if (model) rvs_model_free(model);
            if (error) rvs_error_free(error);

            ctxt = null;
            model = null;
            error = null;
        endfunction

        static local function void handle_error();
            if (rvs_error_test(error)) begin
                $fatal(1, rvs_error_message(error));
            end
        endfunction

        static function void parse(string s);
            if (!ctxt) begin
                $fatal(1, "parse called before initialize() or after transform()");
            end

            rvs_parse(ctxt, s, error);
            handle_error();
        endfunction

        static function void transform();
            rvs_transform(ctxt, model, error);
            handle_error();
            ctxt = null;
        endfunction

        static function bit exists(string name);
            return rvs_get(ctxt, name) > 0;
        endfunction

        static function rvs_handle get(string name);
            rvs_handle handle = rvs_get(model, name);

            if (!handle) begin
                $fatal(1, "Variable '%s' not found", name);
            end

            return handle;
        endfunction

        static function rvs_model get_model();
            return model;
        endfunction

        // Function: seed_from_plusargs
        //
        // Obtains the seed from the +seed= plusarg.  Accepts decimal and
        // hexadecimal (0x prefix) values.
        static function uint32_t seed_from_plusargs(uint32_t default_seed = 0);
            uint32_t seed;

            if ($value$plusargs("seed=0x%h", seed)) begin
                return seed;
            end else if ($value$plusargs("seed=%d", seed)) begin
                return seed;
            end else begin
                return default_seed;
            end
        endfunction

        // Function: search_path_from_plusargs
        //
        // Obtains the search path from the +rvs-search-path= plusarg.
        static function string search_path_from_plusargs(string default_search_path = "");
            string search_path;

            if ($value$plusargs("rvs-search-path=", search_path)) begin
                return search_path;
            end else begin
                return default_search_path;
            end
        endfunction

        // Function: parse_from_plusargs
        //
        // Passes the value of the +rvs= plusarg to Rvs::parse.
        static function void parse_from_plusargs();
            string s;

            if ($value$plusargs("rvs=%s", s)) begin
                parse(s);
            end
        endfunction

        static function write_definitions(string filename);
            rvs_write_definitions(model, filename, error);
            handle_error();
        endfunction
    endclass

    class Rv#(type T = rvs_result);
        local string name;
        local rvs_model model;
        local rvs_handle handle;

        function new(string name);
            this.name = name;
            this.model = Rvs::get_model();
            this.handle = Rvs::get(name);
        endfunction

        function T next();
            return T'(rvs_next(model, handle));
        endfunction

        function T prev();
            return T'(rvs_prev(model, handle));
        endfunction

        function bit done();
            return rvs_done(model, handle);
        endfunction

        function string get_name();
            return name;
        endfunction
    endclass
endpackage
