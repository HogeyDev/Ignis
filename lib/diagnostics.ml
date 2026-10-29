exception Error of string

let error_align_caret line pos =
    let buf = Buffer.create (String.length line) in
    let off = ref 0 in
    String.iteri (fun i c ->
        if c = '\t' then begin
            Buffer.add_string buf "    ";
            if i < pos then off := !off + 4
        end else begin
            Buffer.add_char buf c;
            if i < pos then off := !off + 1
        end) line;
    (Buffer.contents buf, !off)

let report ~filename ~lines ~pos msg =
    let line, col = pos in
    Fmt.epr "%a: %s:%d:%d: %s@."
        Fmt.(styled `Red string) "error" filename (line+1) (col+1) msg;
    let src = lines.(line) in
    let fixed, off = error_align_caret src col in
    Fmt.epr "%s@." fixed;
    Fmt.epr "%s^ %s@." (String.make off ' ') msg;
    raise (Error msg)
