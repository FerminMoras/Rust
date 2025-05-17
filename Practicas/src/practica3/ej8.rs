#[derive(Clone,Debug)]
struct Cancion {
    titulo: String,
    artista: String,
    genero: Genero,
}

struct Playlist {
    lista_canciones: Vec<Cancion>,
    nombre: String,
}

#[derive(Clone,Debug)]
enum Genero {

    ROCK,
    POP,
    RAP,
    JAZZ,
    OTROS,
}

impl Cancion {
    fn new(titulo: String, artista: String, genero: Genero) -> Cancion {
        Cancion {
            titulo: titulo,
            artista: artista,
            genero: genero,
        }
    }

     fn es_mismo_genero(&self, genero: &Genero) -> bool{
        match (&self.genero, genero) {
           (Genero::ROCK, Genero::ROCK) | (Genero::POP, Genero::POP) | (Genero::RAP, Genero::RAP) 
            | (Genero::JAZZ, Genero::JAZZ) | (Genero::OTROS, Genero::OTROS) => {
                true
           }
           _ => false
        }
    }

    fn es_misma_cancion(&self, cancion: &Cancion) -> bool {
        if self.titulo == cancion.titulo && self.artista == cancion.artista && self.es_mismo_genero(&cancion.genero) {
            true
        }
        else {
            false
        }
    }
}

impl Playlist {
    fn new(lista_canciones: Vec<Cancion>, nombre: String) -> Playlist {
        Playlist {
            lista_canciones: lista_canciones,
            nombre: nombre,
        }
    }

    fn agregar_cancion(&mut self, cancion: Cancion) {
       self.lista_canciones.push(cancion); 
    }

    fn eliminar_cancion(&mut self, cancion: Cancion) {
       let mut pos = 0;
       while pos < self.lista_canciones.len() {
            if self.lista_canciones[pos].es_misma_cancion(&cancion) {
                self.lista_canciones.remove(pos);
                break;
            }
            pos += 1;
       } 
    }

    fn mover_cancion(&mut self, posicion: usize, cancion: Cancion) {
        let mut pos = 0;
        while pos < self.lista_canciones.len() {
            if self.lista_canciones[pos].es_misma_cancion(&cancion) {
                let aux = &self.lista_canciones[posicion].clone();
                self.lista_canciones[posicion] = self.lista_canciones[pos].clone();
                self.lista_canciones[pos] = aux.clone();
                break;
            }
            pos += 1;
        }    
    }

    fn buscar_por_nombre(&self, titulo: String) -> Option<Cancion> {
        let mut pos = 0;
        while pos < self.lista_canciones.len() {
            if self.lista_canciones[pos].titulo == titulo {
                return Some(self.lista_canciones[pos].clone());
            }
            pos += 1;
        }
        return None;
    }

    fn cancion_por_genero(&self, genero: Genero) -> Vec<Cancion> {
        let mut vec_nuevo: Vec<Cancion> = Vec::new();
        for i in 0..self.lista_canciones.len() {
            if self.lista_canciones[i].es_mismo_genero(&genero) {
                vec_nuevo.push(self.lista_canciones[i].clone());
            }
        }
        vec_nuevo
    }

    fn cancion_por_artista(&self, artista: String) -> Vec<Cancion> {
        let mut vec_nuevo: Vec<Cancion> = Vec::new();
        for i in 0..self.lista_canciones.len() {
            if self.lista_canciones[i].artista == artista {
                vec_nuevo.push(self.lista_canciones[i].clone());
            }
        }
        vec_nuevo
    }

    fn modificar_titulo_playlist(&mut self, nombre: String) {
        self.nombre = nombre;        
    }

    fn limpiar_playlist(&mut self) {
        self.lista_canciones.clear();
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_cancion(){
        let cancion1 = Cancion::new("Veneno".to_string(), "La renga".to_string(), Genero::ROCK);
        let cancion2 = Cancion::new("Zafar".to_string(), "La Vela Puerca".to_string(), Genero::ROCK);
        let cancion3 = Cancion::new("Habil".to_string(), "Acru".to_string(), Genero::RAP);
        
        let vector = Vec::new();
        let mut playlist = Playlist::new(vector, "Mix".to_string());

        playlist.agregar_cancion(cancion1.clone());
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3.clone());

        playlist.eliminar_cancion(cancion1.clone());
        
        let ok = playlist.lista_canciones[0].es_misma_cancion(&cancion1.clone());
        assert_eq!(ok,false);

        let aux = playlist.buscar_por_nombre("Habil".to_string());
        match aux {
            Some(nombre) => {
                assert_eq!(cancion3.es_misma_cancion(&nombre),true);
            }
            _ => (),
        }
    }    

    #[test]
    fn test_buscar_por_genero() {
        let cancion1 = Cancion::new("Veneno".to_string(), "La renga".to_string(), Genero::ROCK);
        let cancion2 = Cancion::new("Zafar".to_string(), "La Vela Puerca".to_string(), Genero::ROCK);
        let cancion3 = Cancion::new("Habil".to_string(), "Acru".to_string(), Genero::RAP);
        
        let vector = Vec::new();
        let mut playlist = Playlist::new(vector, "Mix".to_string());

        playlist.agregar_cancion(cancion1.clone());
        playlist.agregar_cancion(cancion2.clone());
        playlist.agregar_cancion(cancion3);

        let lista = playlist.cancion_por_genero(Genero::ROCK);
        for i in 0..lista.len() {
            assert_eq!(lista[i].es_misma_cancion(&lista[i]),true);
        }    
    }

    #[test]
    fn test_buscar_por_artista() {
        let cancion1 = Cancion::new("Veneno".to_string(), "La renga".to_string(), Genero::ROCK);
        let cancion2 = Cancion::new("Zafar".to_string(), "La Vela Puerca".to_string(), Genero::ROCK);
        let cancion3 = Cancion::new("Habil".to_string(), "Acru".to_string(), Genero::RAP);
        let cancion4 = Cancion::new("Roman".to_string(), "Acru".to_string(), Genero::RAP);
        let cancion5 = Cancion::new("Hattori Hanzo".to_string(), "Acru".to_string(), Genero::RAP);
        
        let vector = Vec::new();
        let mut playlist = Playlist::new(vector, "Mix".to_string());

        playlist.agregar_cancion(cancion1);
        playlist.agregar_cancion(cancion2);
        playlist.agregar_cancion(cancion3.clone());
        playlist.agregar_cancion(cancion4.clone());
        playlist.agregar_cancion(cancion5.clone());

        let lista = playlist.cancion_por_artista("Acru".to_string());
        for i in 0..lista.len() {
            assert_eq!(lista[i].es_misma_cancion(&lista[i]),true);
        }

        playlist.limpiar_playlist();
        let aux = playlist.buscar_por_nombre("Habil".to_string());
        match aux {
            Some(nombre) => {
                assert_eq!(cancion3.es_misma_cancion(&nombre),true);
            }
            _ => (),
        }
    }

    #[test]
    fn test_modificar_titulo() {
        let vector = Vec::new();
        let mut playlist = Playlist::new(vector, "Playlist Fermin".to_string());

        playlist.modificar_titulo_playlist("Mix".to_string());
        assert_eq!(playlist.nombre, "Mix".to_string());
    }

}