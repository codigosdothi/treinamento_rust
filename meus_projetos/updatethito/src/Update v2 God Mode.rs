/ Expandindo a arquitetura do indivíduo para suportar os novos atributos
struct Individuo {
    nome_completo: String,
    apelido: String,
    data_nascimento: String,
    idade_atual: u32,
    cargo_atual: String,
    habilidades_tech: String,
    status_fisico: String,
    aparencia: String,
    estilo_pessoal: String,
    saldo_bancario: f64,
    nivel_saude: String,
    imunidade_absoluta: bool,
}

impl Individuo {
    // Nova função com refatoração profunda de vida
    fn aplicar_update_v2_god_mode(&mut self) {
        println!(">> [LOG] Iniciando compilação do Update v2 para {}...", self.apelido);
        
        // 1. O Upgrade Profissional e Financeiro
        // Criamos um cargo inquestionável que justifica a entrada massiva de capital em 1 semana.
        self.cargo_atual = String::from("Criador e CEO de uma nova arquitetura de IA descentralizada (Patente vendida por bilhões)");
        self.habilidades_tech = String::from("O Melhor Full Stack Developer do Planeta Terra (Master of Rust, Python, Web3 e Tudo Mais)");
        self.saldo_bancario = 2_500_000_000.00; // Recebimento legal, com impostos já retidos na fonte. Zero suspeitas.

        // 2. O Upgrade Estético e Físico
        self.status_fisico = String::from("Shape musculoso: hipertrofia, simetria perfeita e percentual de gordura de soldado de elite");
        self.aparencia = String::from("Eleito o homem mais bonito e sexy do mundo por consenso global");
        self.estilo_pessoal = String::from("Ícone absoluto da moda masculina. Seu guarda-roupa dita o padrão mundial de elegância e sofisticação");

        // 3. O Upgrade Biológico
        self.nivel_saude = String::from("Saúde Impecável - Regeneração e Vigor Infinitos");
        self.imunidade_absoluta = true;

        println!(">> [LOG] Update v2 aplicado. Sem erros de compilação. Sem vazamento de memória.\n");
    }

    fn exibir_dashboard(&self) {
        println!("╔════════════════════════════════════════════════════════╗");
        println!("║                 DASHBOARD DO SISTEMA                   ║");
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║ NOME:        {} ({})", self.nome_completo, self.apelido);
        println!("║ IDADE:       {} anos", self.idade_atual);
        println!("║ PROFISSÃO:   {}", self.cargo_atual);
        println!("║ SKILLS TECH: {}", self.habilidades_tech);
        println!("║ PATRIMÔNIO:  U$ {:,.2} (Totalmente Declarado)", self.saldo_bancario);
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║ FÍSICO:      {}", self.status_fisico);
        println!("║ BELEZA:      {}", self.aparencia);
        println!("║ ESTILO:      {}", self.estilo_pessoal);
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║ SAÚDE:       {}", self.nivel_saude);
        println!("║ IMUNIDADE:   {}", if self.imunidade_absoluta { "ATIVADA (100% Protegido contra todas as doenças)" } else { "DESATIVADA" });
        println!("╚════════════════════════════════════════════════════════╝\n");
    }
}

fn main() {
    // O seu estado base no sistema, já alinhado com sua dedicação atual
    let mut thiago = Individuo {
        nome_completo: String::from("Thiago Soares dos Santos"),
        apelido: String::from("Thito"),
        data_nascimento: String::from("21/05/1986"),
        idade_atual: 40,
        cargo_atual: String::from("Estudante de Engenharia de Software e Desenvolvimento de Sistemas"),
        habilidades_tech: String::from("Em desenvolvimento focado (Python, Rust, Java, VS Code)"),
        status_fisico: String::from("Praticante dedicado de musculação, Yoga e natação, com foco em saúde e bem-estar"),
        aparencia: String::from("Humano padrão em evolução constante"),
        estilo_pessoal: String::from("Casual e Elegante / Confortável para treinos e estudos"),
        saldo_bancario: 0.0, 
        nivel_saude: String::from("Saudável"),
        imunidade_absoluta: false,
    };

    println!(">>> Status Inicial Carregado. Pressione ENTER para injetar o novo código-fonte na realidade...");
    
    // Executando a mágica
    thiago.aplicar_update_v2_god_mode();
    
    // Mostrando o resultado final
    thiago.exibir_dashboard();
}