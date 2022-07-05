CÓDIGO IDIOMÁTICO

Significa programar de forma natural, ou seja ser fluente em Rust, no caso.
    
    COMANDOS:

    Rustfmt -> formata o código para você, com o: 

        cargo fmt
    
    assim o código recebe automáticamente toda a identação e quebra de linhas para leitura do código.

    Clippy -> avisa sobre erros de boas práticas, com o comando:

        cargo clippy
        
    Erros como:

        Estilo, por exemplo usando o return ao invés de uma expressão de retorno.

        Correção, como quando um trecho de código é completamente inútil. Por exemplo, um loop que não tem nada além de um break dentro.

        Complexidade, quando o código está complexo sem necessidade, como avisar sobre parenteses demais ou uma função com muitos argumentos, por exemplo.

        Performance, ele irá avisar quando algo estiver deixando o código lento sem necessidade. Por exemplo, uma atribuição de váriavel que quer o tamanho de um vetor sem usar a função interator.count(), mas pedindo para primeiro clonar, coletar o vetor e então dizer seu tamanho. Algo que irá levar mais tempo do que uma atribuição simples que faz a mesma coisa.
