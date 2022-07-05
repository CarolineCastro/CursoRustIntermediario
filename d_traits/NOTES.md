TRAÇOS COMUNS

O que pode implementar uma trait?
    Structs, Enums, Closures e Funções.

        OBS: Closures e Funções só podem implementar algumas traits avançadas.

    DERIVABLE TRAITS

    Uma trait pode ser derivada se houver derivada uma macro definida por ela.

        EXEMPLO 1: Debug 

            #[derive(Debug)]

            pub struct Puzzle {
                pub num_pieces: u32,
                pub name: String,
            }

            println!("{:?}", puzzle);
            //print Puzzle {num_pieces: 500, name: "Draconic Equestian"}

            println!("{:#?}", puzzle)
            //print Puzzle{
                num_pieces: 500,
                name: "Draconic Equestrian",
            }
        
        EXEMPLO 2: Clone

            
            #[derive(Clone,Debug)]

            pub struct Puzzle {
                pub num_pieces: u32,
                pub name: String,
            }

            let puzzle 2 = puzzle.clone()

        EXEMPLO 3: Copy

            Não conseguimos implementar o Copy na struct Puzzle porque a String não é Copy, então vamos usar o enum.

            #[derive(Copy)]

            pub enum PuzzleType{
                Jigsaw,
            }

            Esse enum tem uma única váriavel, por isso podemos usar o Copy. Mas para usá-la é preciso derivar o Clone, pois uma depende da outra.

            
            #[derive(Clone, Copy)]

            pub enum PuzzleType{
                Jigsaw,
            }



