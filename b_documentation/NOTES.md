DOCUMENTAÇÃO

Como documentar o seu código rust. Primeiro iremos usar o comando:

    cargo doc --no-deps --open
    
Para gerar uma página html para a nossa documentação que é o que está declarado na lib.rs

Para começar a escrever uma domentação existem alguns formas de formatação. Tudo que for domentação estará numa notação diferente de comentário. 

    /// texto
    ^ Esse tipo define um parágrafo, sem nada do lado é uma quebra de parágrafo.

    /// # Título
        ^ Esse tipo define um título.
    
    /// link ['nome_struct']
             ^hiperlink para uma struct, pode ser escrito sem mostrar o nome, como um texto clicável
             /// [link](nome_struct)
    
    //! texto topo
      ^ Usado para criar um header para a documentação.


