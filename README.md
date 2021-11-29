# PathfinderBattleManager
Um programa com utilidade prática. Um gerenciador de batalha para um sistema de RPG. Ainda está em construção.

O programa mesmo fica na pasta "campaing_manager", utilizando uma biblioteca de de GUI imediata (Imgui) sendo renderizado pela "render_api".

Por enquanto o programa apenas organiza e renderiza as informações de diversas criaturas disponível num banco de dados em json (https://gitlab.com/hooking/foundry-vtt---pathfinder-2e/-/tree/master/packs/data). Foi aqui que pensei em utilizar SQLite. Ingerir os json e criar um objeto de cada criatura demorava mais de 3 min (44 s com multithread), então pensei que SQL poderia me ajudar e comecei a estudar, mas depois de bastante dificuldade com o schema, já que são dados bem complexos e pouco uniformes, cheguei a solução de serializar um array de todos os objetos e carregar ele no carregamento do programa. Esse array carrega em alguns décimos de segundo e fica perfeito para o debug do programa enquanto eu trabalho nele.

A serialização é feito pelo "blob_creator", que consome os json, cria os objetos e serializa eles em binário, tudo usando multithread.

Esses objetos são definidos pela biblioteca "Creature" contendo uma "classe" (em Rust, tudo é struct) que define as propriedades e metodos de criação e modificação dessas criaturas.

Por fim, "parser" pega descrições de diversos itens dentro de cada criatura (Magias e Habilidades, por exemplo) que contem divesas Tag de formatação e outros comandos uteis para a aplicação à qual o banco de dados foi criado, e me entrega instruções (uma enumeração) que são utilizadas para formatar essa descrição. 




