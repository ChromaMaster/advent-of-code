let file = "examples/example-day02.txt"

let get_left_and_right_lists : string list * string list =
  let rec build_lists left right lines =
    match lines with
    | first :: rest ->
        let l, r = Str.split (Str.regexp "[ \t]+") first in
        build_lists (l :: left) (r :: right) rest
    | [] -> (List.rev left, List.rev right)
  in
  (* build_lists [] [] Common.read_file file *)
  build_lists [] [] [ "first second"; "third fourth" ]

(* let part_one () = List.iter print_endline (Common.read_file file) *)

let l, r = get_left_and_right_lists
let part_one () = List.iter print_endline l
(* let () = print_endline "Hello, World!" *)
