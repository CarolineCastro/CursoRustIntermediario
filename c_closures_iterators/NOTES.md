FECHAMENTOS

Uma closure (fechamento) é uma função anônima que pode emprestar ou capturar algum dado do escopo em que está aninhado.

A sintaxe para uma closure é:

    |paramns| expression

O compilador descobre os tipos dos argumentos e o tipo do retorno pela forma em que você usa a closure.

Vamos usar uma clousure em uma variável de nome add e chamá-la com alguns números como argumentos:

    let add = |x, y| x + y;

    add (1, 2); // return 3;

Voltando a sintaxe da nossa closure. Não precisamos obrigatóriamente de parâmentros, você pode deixar o campo de parâmetros em branco.

    || x + y

Tecnicamente, você não precisa de uma expressão, pode usar um bloco vazio para o corpo também.

    || {}

Isso é uma sintaxe válida para closure que não é particularmente interessante. O que é realmente interessante é uma closure que vai automáticamente emprestar uma referência de valores em um escopo fechado.

EXEMPLO:

    let s = "x";

    let f = || {
        println!("{}", s);
    };

    f(); //prints x

    Aqui criamos uma string s e uma closure que empresta a referência para s, então atribuimos a a closure para a variável f e sempre que chamarmos f, irá printar um "x".
    Esse comportamento de empréstimo-por-padrão da closure é ótima desde que a closure seja descartada antes do valor que está sendo referenciado seja descartado.
    Isso não irá compilar se o compilador nao consegue garantir que o valor irá viver tanto quanto a closure que o está referenciando.
    Para nossa sorte, closures suportam a semântica move adicionando a palavra chave move antes da closure. Isso fará a closure mover qualquer variáveis que está sendo usada para dentro de si mesma e se tornar a dona dela.

        let f = move || {
            println!("{}", s);
        }

        Agora s pertence a clouse, e só será descartada quando a própria closure sair do escopo e ser descartada.
        Então nós podemos mandar essa closure para outra thred ou retorná-la para uma função sem nos preocuparmos com o descarte de s.
        Claro que isso implica em não precisar acessar s em outro escopo. Se for o caso, você irá precisar desse valor fora da closure também, então uma outra opção é clonar o s e passar o clone para a closure com o move.

ITERADORES

Em loops For irão iterar sobre qualquer valor iterável. Isso porque loops For usam iteradores sob o capô. Mas de onde esse iterador vem?

    let v = vec![6, 7, 8, 9];
    for num in v{
        println!("{}", num);
    }

    O vetor v não está sendo usando diretetamente pelo loop. O loop For opera sob qualquer iterador, e se não for um itereador, ele chama o método into_iter(), que retorna o atual iterador. 

        for num in v.into_iter() {
            println!("{}", num);
        }
    
        into_iter() é um método do traço IntoIterator.
        Qualquer coisa implemnetada pelo IntoInterator será convertido para dentro de um iterador pelo o loop For automaticamente. O into_inter() retorna um iterador que assume a propriedade da coleção que está implementando, consumindo a coleção.
        Por isso, até então, fomos cuidadosos ao usar loops For em vetores que não seriam necessários por muito tempo, porque depois do vetor ser usando num loop For ele é descartado.

    Agora iremos escrever um loop For para um vetor da forma correta, de acordo com as boas práticas.

        let v = vec![6, 7, 8];

        v.into_iter().for_each(|num| println!("{}", num));

        Dessa vez iremos chamar explicitamente o into_iter() para obter o iterador, e depois chamar o iterador for_each. Esse método pega uma closure que aceita um valor do vetor como argumento. Nossa closure faz o mesmo println que fazia no loop For.

        A razão de usar o iterador é que eles são mais rápidos que os loops For, outra razão é que se pode tirar vantagem das adaptações do iterador. 

    ITERATOR ADAPTERS
  
    Uma adaptação de iterador é uma ferramenta do paradigma da programação funcional que assume um interador e retorna um iterador diferente que irá assumir alguma ação nos valores que foram passados para ele.
    Muitos dos métodos do iterador, são adaptadores de iterador.

    ITERATOR CONSUMER

    Iteradores são preguiçosos e não fazem nada se não forem consumidos, por isso temos os iterator consumer.
    Um Iterator Consumer é algo que consume o final do iterador de alguma forma fazendo com que a cadeira de iterator adapters façãm seu processamento.
    O for_each() é um iterator consumer que consome cada valor e os passa para uma closure cujo o valor é descartado.