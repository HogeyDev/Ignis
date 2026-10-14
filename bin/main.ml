let () =
    let path = Sys.argv.(1) in
    let contents = In_channel.with_open_text path In_channel.input_all in
    prerr_endline contents
