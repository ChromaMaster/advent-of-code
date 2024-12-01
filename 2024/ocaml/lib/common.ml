let read_file filename : string list =
  let lines = ref [] in
  let chan = open_in filename in
  let rec read_next () =
    try
      let line = input_line chan in
      lines := line :: !lines;
      read_next ()
    with End_of_file ->
      close_in chan;
      List.rev !lines
  in
  read_next ()
