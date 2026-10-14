type t = {
    filename : string;
    file_id : int;
    source : string;
    lines : string array;
    mutable i : int; (* byte offset into source *)
    mutable line : int;
    mutable col : int;
}

let from ~filename ~file_id ~source =
    let lines = String.split_on_char '\n' source in
    { filename; file_id; source; lines = Array.of_list lines; i = 0; line = 0; col = 0 }

let len t = String.length t.source
let get t i = if i >= 0 && i < len t then Some t.source.[i] else None
let curr t = get t t.i
let rel t off = get t (t.i + off)
let advance t =
    (match curr t with
     | Some '\n' -> t.line <- t.line + 1; t.col <- 0
     | _ -> t.col <- t.col + 1);
    t.i <- t.i + 1

let skip_whitespace t =
    while (match curr t with
           | Some c when Char.Ascii.is_white c -> true
           | _ -> false) do
        advance t
    done

let skip_comment t =
    while (match curr t with
           | Some '\n' | None -> false
           | _ -> true) do
        advance t
    done

let identifier t =
    let b = Buffer.create 16 in
    Buffer.add_char b (Option.get (curr t));
    advance t;
    while (match curr t with
           | Some c when Char.Ascii.is_alphanum c || c == '_' -> true
           | _ -> false) do
        Buffer.add_char b (Option.get (curr t));
        advance t
    done;
    let s = Buffer.contents b in
    match s with
    | "continue" -> Token.Continue
    | "func" ->     Token.Function
    | "type" ->     Token.TypeDef
    | "import" ->   Token.Import
    | "return" ->   Token.Return
    | "static" ->   Token.Static
    | "struct" ->   Token.Struct
    | "break" ->    Token.Break
    | "defer" ->    Token.Defer
    | "while" ->    Token.While
    | "cast" ->     Token.Cast
    | "else" ->     Token.Else
    | "enum" ->     Token.Enum
    | "spec" ->     Token.Spec
    | "asm" ->      Token.Asm
    | "for" ->      Token.For
    | "let" ->      Token.Let
    | "if" ->       Token.If
    | "void" | "char" | "usize" | "isize"
    | "u64" | "u32" | "u16" | "u8"
    | "i64" | "i32" | "i16" | "i8" -> Token.PrimType s
    | "Func" ->     Token.FuncType
    | _ -> Ident s

(* exception Error of string

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
    Fmt.(styled `Red string) "error" filename (line + 1) (col + 1) msg;
  let src = lines.(line) in
  let fixed, off = error_align_caret src col in
  Fmt.epr "%s@." fixed;
  Fmt.epr "%s^ %s@." (String.make off ' ') msg;
  raise (Error msg) *)

let number t =
    let b = Buffer.create 16 in
    while (match curr t with
           | Some c when Char.Ascii.is_digit c -> true
           | _ -> false) do
        Buffer.add_char b (Option.get (curr t));
        advance t
    done;
    let value = Buffer.contents b in
    let suffix =
        (match curr t with
         | Some ('i' | 'u') ->
             (match identifier t with
              | Token.PrimType x -> x
              | _ -> Diagnostics.report ~filename:t.filename ~lines:t.lines ~pos:(t.line, t.col) "expected a primitive integer type")
         | _ -> "i32")
    in Token.Integer (value, suffix)
