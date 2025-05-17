use super::ej3::Fecha;
enum Genero {
    NOVELA,
    INFANTIL,
    TECNICO,
    OTROS
}

enum Estado {
    DEVUELTO,
    PRESTAMO,
}

struct Cliente{
    nombre: String,
    telefono: u32,
    email: String
}
impl Cliente{

    fn new(nombre: String, telefono: u32, email: String) -> Cliente{
        Cliente{
            nombre,
            telefono,
            email
        }
    }

    fn es_mismo_cliente(&self, cli: &Cliente) -> bool{
        if(self.nombre == cli.nombre) && (self.telefono == cli.telefono) && (self.email == cli.email){
            true
        }else{
            false
        }
    }
}

struct  Libro{
    isbn: u32,
    titulo: String,
    autor: String,
    nro_paginas: u32,
    genero: Genero,
}
impl Libro{

    fn new(isbn: u32, titulo: String, autor: String, nro_paginas: u32, genero: Genero) -> Libro {
        Libro {
            isbn,
            titulo,
            autor,
            nro_paginas,
            genero,
        }
    }

    fn es_mismo_genero(&self, tipo_genero: &Genero) -> bool{
        match (&self.genero, tipo_genero) {
            (Genero::NOVELA, Genero::NOVELA) | (Genero::INFANTIL, Genero::INFANTIL) | (Genero::TECNICO, Genero::TECNICO) | (Genero::OTROS, Genero::OTROS) => true,
            _ => false,
        }
    }

    fn es_mismo_libro(&self, libro: &Libro) -> bool{
        if(self.isbn == libro.isbn) && (self.titulo == libro.titulo) && (self.autor == libro.autor) && (self.nro_paginas == libro.nro_paginas) && (self.es_mismo_genero(&libro.genero)){
            true
        }else{
            false
        }
    }
}

struct Disposicion{
    isbn: u32,
    cantidad: u32,
}
impl Disposicion{
    fn new(isbn: u32, cantidad: u32) -> Disposicion {
        Disposicion {
            isbn,
            cantidad,
        }
    }
}

// Esto le dice al compilador: "Prestamo va a contener referencias a un Libro y un Cliente que deben vivir al menos tanto como el Prestamo".
struct Prestamo<'a> {
    libro: &'a Libro,
    cliente: &'a Cliente,
    fecha_vencimiento: Fecha,
    fecha_devolucion: Option<Fecha>,
    estado: Estado,
}
impl<'a> Prestamo<'a> {
    fn new(libro: &'a Libro, cliente: &'a Cliente, fecha_vencimiento: Fecha, fecha_devolucion: Option<Fecha>, estado: Estado) -> Prestamo<'a> {
        Prestamo {
            libro,
            cliente,
            fecha_vencimiento,
            fecha_devolucion,
            estado,
        }
    }
    fn es_mismo_estado(&self, tipo_estado: &Estado) -> bool{
        match (&self.estado, tipo_estado) {
            (Estado::PRESTAMO, Estado::PRESTAMO) => true,
            _ => false,
        }
    }
}

struct Biblioteca<'a> {
    nombre: String,
    direccion: String,
    libros: Vec<&'a Libro>,
    copias_a_prestar: Vec<Disposicion>,
    prestamos_efectuados: Vec<Prestamo<'a>>,
}
impl<'a> Biblioteca<'a>{
    fn new(nombre: String, direccion: String, libros: Vec<&'a Libro>, copias_a_prestar: Vec<Disposicion>, prestamos_efectuados: Vec<Prestamo<'a>>) -> Biblioteca<'a> {
        Biblioteca {
            nombre,
            direccion,
            libros,
            copias_a_prestar,
            prestamos_efectuados,
        }
    }

    //➔​ obtener cantidad de copias: dado un determinado libro retorna la cantidad de copias a disposición que hay para prestar de dicho libro.
    fn obtener_cantidad_de_copias(&self, libro: &Libro) -> u32{
        for disposicion in &self.copias_a_prestar{
            if disposicion.isbn == libro.isbn {
                return disposicion.cantidad
            }
        }
        return 0;
    }

    //➔​ decrementar cantidad de copias a disposición; dado un libro decrementa en 1 la cantidad de copias de libros a disposición para prestar.
    fn decrementar_cantidad_de_copias_a_disposicion(& mut self, libro: &Libro){
        for i in 0..self.copias_a_prestar.len(){
            if self.copias_a_prestar[i].isbn == libro.isbn && self.copias_a_prestar[i].cantidad > 0{
                self.copias_a_prestar[i].cantidad = self.copias_a_prestar[i].cantidad - 1;
                break
            }
        }
    }   

    //➔​ incrementar cantidad de copias a disposición: dado un libro incrementa en 1 la cantidad de copias del libro a disposición para ser prestado.
    fn incrementar_cantidad_de_copias_a_disposicion(& mut self, libro: &Libro){
        for i in 0..self.copias_a_prestar.len(){
            if self.copias_a_prestar[i].isbn == libro.isbn{
                self.copias_a_prestar[i].cantidad = self.copias_a_prestar[i].cantidad + 1;
                break
            }
        }
    }

    //➔​ contar préstamos de un cliente: devuelve la cantidad de préstamos en estado “en préstamo” de un determinado cliente.
    fn contar_prestamos_de_un_cliente(&self, cli: &Cliente) -> u32{
        let mut contador: u32 = 0;
        for prestamo in &self.prestamos_efectuados{
            if(prestamo.cliente.es_mismo_cliente(cli)) && (prestamo.es_mismo_estado(&Estado::PRESTAMO)){
                contador = contador + 1;
            }
        }   
        return contador;
    }

    /*
    ➔​ realizar un préstamo de un libro para un cliente: crea un préstamo de un libro para un determinado cliente cumpliendo con lo siguiente
    ◆​ el cliente no tenga más de 5 préstamos en el estado “en préstamo”
    ◆​ haya al menos una copia disponible en el registro de copias a
    disposición.
    De ser así descuenta 1 en el registro de “copias a disposición” y retorna true, si no cumple con alguna de las condiciones retorna false.
    */
    fn realizar_prestamo(& mut self, libro: &'a Libro, cli: &'a Cliente, fecha_vencimiento: Fecha, fecha_devolucion: Option<Fecha>) -> bool{
        let prestamos_del_cliente: u32 = self.contar_prestamos_de_un_cliente(cli);
        let cantidad_de_copias: u32 = self.obtener_cantidad_de_copias(libro);
        if(prestamos_del_cliente < 5) && (cantidad_de_copias > 0){
            let nuevo_prestamo = Prestamo::new(libro, cli, fecha_vencimiento, fecha_devolucion, Estado::PRESTAMO);
            self.prestamos_efectuados.push(nuevo_prestamo);
            self.decrementar_cantidad_de_copias_a_disposicion(libro);
            return true
        }else{
            return false
        }
    }

    //➔​ ver préstamos a vencer el los próximos días: retorna una lista de préstamos a vencer el los próximos días, el valor de días es pasado por parámetro.
    fn prestamos_a_vencer(&self, hoy: &Fecha, dias: u16) -> Vec<&Prestamo<'a>>{  //retorno una lista de refencias a prestamos que viven tanto como self.prestamos_efectuados
        let mut lista_a_vencer: Vec<&Prestamo> = Vec::new();
        
        let mut fecha_limite: Fecha = Fecha::new(hoy.dia, hoy.mes, hoy.año);
        fecha_limite.sumar_dias(dias);
        
        for prestamo in &self.prestamos_efectuados{
            if (prestamo.es_mismo_estado(&Estado::PRESTAMO)) && (!hoy.es_mayor(&prestamo.fecha_vencimiento)) && (!prestamo.fecha_vencimiento.es_mayor(&fecha_limite)){  //si la fecha del dia de hoy no es mayor a la fecha del vencimiento, quiere decir que el prestamo vencio
                lista_a_vencer.push(prestamo);
            }
        }
        return lista_a_vencer
    }

    //➔​ ver los préstamos vencidos: retorna una lista de préstamos en el estado “en préstamos” donde la fecha de vencimiento es menor a la fecha actual.
    fn prestamos_vencidos(&self, hoy: &Fecha) -> Vec<&Prestamo<'a>>{
        let mut lista_vencida: Vec<&Prestamo> = Vec::new();
        
        for prestamo in &self.prestamos_efectuados{
            if (prestamo.es_mismo_estado(&Estado::PRESTAMO)) && (hoy.es_mayor(&prestamo.fecha_vencimiento)){  //si la fecha del dia de hoy no es mayor a la fecha del vencimiento, quiere decir que el prestamo vencio
                lista_vencida.push(prestamo);
            }
        }
        return lista_vencida
    }

    //➔​ buscar préstamo: dado un libro y un cliente busca un préstamo y lo retorna si existe.    //preguntar si esta bien
    fn buscar_prestamo(&self, libro: &Libro, cli: &Cliente) -> Option<&Prestamo<'a>>{  //referencia inmutable
        for pos in 0..self.prestamos_efectuados.len(){
            if (self.prestamos_efectuados[pos].libro.es_mismo_libro(&libro)) && (self.prestamos_efectuados[pos].cliente.es_mismo_cliente(&cli)){
                return Some(&self.prestamos_efectuados[pos]);
            }
        }
        return None
    }

    //➔​ devolver libro: dado un libro y un cliente se busca el préstamo y se cambia al estado “devuelto”, se registra la fecha de devolución y se incrementa la cantidad de libros en 1 del libro devuelto en el registro de copias a disposición.
    fn devolver_libro(& mut self, libro: &Libro, cli: &Cliente, hoy: &Fecha){
        for prestamo in &mut self.prestamos_efectuados {
            if prestamo.libro.es_mismo_libro(libro) && prestamo.cliente.es_mismo_cliente(cli) {
                prestamo.estado = Estado::DEVUELTO;
                prestamo.fecha_devolucion = Some(Fecha::new(hoy.dia, hoy.mes, hoy.año));
                self.incrementar_cantidad_de_copias_a_disposicion(libro);
                break;
            }
        }
    }
}

#[test]
fn todos_los_test_aqui(){
    let fecha_vencimiento0: Fecha = Fecha::new(16, 5, 2025);
    let fecha_vencimiento1: Fecha = Fecha::new(31, 6, 2025);
    let fecha_vencimiento2: Fecha = Fecha::new(17, 7, 2025);
    let fecha_vencimiento3: Fecha = Fecha::new(6, 8, 2025);
    let fecha_vencimiento4: Fecha = Fecha::new(10, 10, 2025);
    let cliente0: Cliente = Cliente::new("Juan Roman Riquelme".to_string(), 123456, "juanromanmate@email.com".to_string());
    let cliente1: Cliente = Cliente::new("Chicho serna".to_string(), 123456, "boquitaserna@email.com".to_string());
    let cliente2: Cliente = Cliente::new("Cascini Mosquito".to_string(), 123456, "elmosquito@email.com".to_string());
    let cliente3: Cliente = Cliente::new("Tomas Agustin Pisani".to_string(), 123456, "aguantebocapedazodequebrado@email.com".to_string());
    //Crear vector de libros
    let mut libros: Vec<&Libro> = Vec::new();
    let libro0: Libro = Libro::new(123, "SOY LIBRO 1".to_string(), "tomasaaaaa".to_string(), 3300, Genero::INFANTIL);
    libros.push(&libro0);
    let libro1: Libro = Libro::new(124, "SOY LIBRO 2".to_string(), "tomasddd".to_string(), 320, Genero::TECNICO);
    libros.push(&libro1);
    let libro2: Libro = Libro::new(125, "SOY LIBRO 3".to_string(), "tomasdsa".to_string(), 3030, Genero::OTROS);
    libros.push(&libro2);
    let libro3: Libro = Libro::new(126, "SOY LIBRO 4".to_string(), "tomas pisani".to_string(), 400, Genero::TECNICO);
    libros.push(&libro3);
    let libro4: Libro = Libro::new(127, "SOY LIBRO 5".to_string(), "tomas pisani".to_string(), 5060, Genero::OTROS);
    libros.push(&libro4);
    // Crear disposiciones (cantidad de copias disponibles para prestar)
    let mut disposiciones: Vec<Disposicion> = Vec::new();
    let disposicion0: Disposicion = Disposicion::new(123, 3);
    disposiciones.push(disposicion0);
    let disposicion1: Disposicion = Disposicion::new(124, 5);
    disposiciones.push(disposicion1);
    let disposicion2: Disposicion = Disposicion::new(125, 0);
    disposiciones.push(disposicion2); // No hay copias disponibles
    let disposicion3: Disposicion = Disposicion::new(126, 2);
    disposiciones.push(disposicion3);
    let disposicion4: Disposicion = Disposicion::new(127, 1);
    disposiciones.push(disposicion4);
    // Crear préstamos efectuados
    let mut prestamos: Vec<Prestamo> = Vec::new();
    let prestamo0: Prestamo = Prestamo::new(&libro0, &cliente0, fecha_vencimiento0, None, Estado::PRESTAMO);
    prestamos.push(prestamo0);
    let prestamo1: Prestamo = Prestamo::new(&libro1, &cliente1, fecha_vencimiento1, None, Estado::PRESTAMO);
    prestamos.push(prestamo1);
    let prestamo2: Prestamo = Prestamo::new(&libro2, &cliente2, fecha_vencimiento2, None, Estado::PRESTAMO);
    prestamos.push(prestamo2);
    let prestamo3: Prestamo = Prestamo::new(&libro3, &cliente3, fecha_vencimiento3, None, Estado::PRESTAMO);
    prestamos.push(prestamo3);
    let prestamo4: Prestamo = Prestamo::new(&libro4, &cliente3, fecha_vencimiento4, None, Estado::PRESTAMO);
    prestamos.push(prestamo4);
    let mut biblioteca: Biblioteca = Biblioteca::new("Biblioteca El Libretazo".to_string(),"Calle Falsa al 9910".to_string(),libros,disposiciones, prestamos);

    //testar obtener cantidad de copias
    assert_eq!(biblioteca.obtener_cantidad_de_copias(&libro1), 5);

    //decrementa la cantidad de copias para prestar
    biblioteca.decrementar_cantidad_de_copias_a_disposicion(&libro1);
    assert_eq!(biblioteca.copias_a_prestar[1].cantidad, 4);  //posicion del libro en el vector

    //incrementa la cantidad de copias para prestar
    biblioteca.incrementar_cantidad_de_copias_a_disposicion(&libro4);
    assert_eq!(biblioteca.copias_a_prestar[4].cantidad, 2);  //posicion del libro en el vector

    //contar prestamos de un cliente
    let contador: u32 = biblioteca.contar_prestamos_de_un_cliente( &cliente1);
    assert_eq!(contador, 1);

    //realizar prestamo valido
    let fecha_vencimiento: Fecha = Fecha::new(28, 9, 2025);
    let fecha_devolucion: Fecha = Fecha::new(29, 9, 2025);
    biblioteca.realizar_prestamo(&libro1, &cliente1, fecha_vencimiento, Some(fecha_devolucion));
    assert_eq!(biblioteca.prestamos_efectuados.len(), 6);

    //realizar prestamo invalido
    let fecha_vencimiento: Fecha = Fecha::new(22, 9, 2025);
    let fecha_devolucion: Fecha = Fecha::new(19, 3, 2026);
    biblioteca.realizar_prestamo(&libro2, &cliente1, fecha_vencimiento, Some(fecha_devolucion));
    assert_eq!(biblioteca.prestamos_efectuados.len(), 6);  //misma longitud por que no se agrega

    //prestamos a vencer (por ejemplo, próximos 50 días desde 16/5/2025)
    let hoy = Fecha::new(16, 5, 2025);
    let prestamos_a_vencer = biblioteca.prestamos_a_vencer(&hoy, 50); // fecha límite: 5/7/2025
    assert_eq!(prestamos_a_vencer.len(), 2);

    //préstamos vencidos al dia 9/10/2025 (solo deberían estar vencidos los libros 0,1,2,3)
    let hoy_vencido = Fecha::new(9, 10, 2025);
    let prestamos_vencidos = biblioteca.prestamos_vencidos(&hoy_vencido);
    assert_eq!(prestamos_vencidos.len(), 5);

    //buscar prestamo existente(libro1, cliente1)
    let prestamo_encontrado = biblioteca.buscar_prestamo(&libro1, &cliente1);
    if prestamo_encontrado.is_some() {
        let prestamo = prestamo_encontrado.unwrap();
        assert_eq!(prestamo.libro.isbn, 124);
        assert_eq!(prestamo.cliente.email, "boquitaserna@email.com");
    }

    //buscar prestamo que no existe(libro1, cliente1)
    let prestamo_inexistente = biblioteca.buscar_prestamo(&libro2, &cliente0);
    let mut encontrado: bool = true;
    if prestamo_inexistente.is_none(){
        encontrado = false;
        assert_eq!(encontrado, false)
    }

    // ➔ devolver libro (libro1 de cliente1)
    let hoy_devolucion = Fecha::new(16, 5, 2025);
    biblioteca.devolver_libro(&libro1, &cliente1, &hoy_devolucion);
    let prestamo_actualizado = biblioteca.buscar_prestamo(&libro1, &cliente1).unwrap();
    match prestamo_actualizado.estado {
        Estado::DEVUELTO => (),
        _ => panic!("El préstamo no fue marcado como devuelto"),
    }
}
