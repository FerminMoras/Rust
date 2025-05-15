struct Persona{
    nombre: String,
    edad: i32,
    direccion: Option<String>, 
}

impl Persona{
    fn new(name:String, age: i32, direc: Option<String>) -> Persona{
      Persona {
        nombre: name.to_string(),
        edad: age,
        direccion: direc,
      }  
    }

    fn to_string(&self) -> String{
        match &self.direccion{
            Some(direc) => format!(
                "Hola, me llamo {}, tengo {} años y vivo en {}", self.nombre, self.edad, direc
            ),
            None => format!(
                "Hola, me llamo {}, tengo {} años y no tengo una direccion asignada", self.nombre, self.edad
            ) 
        }
            
    }

    fn obtener_edad(self) -> i32 {
        return self.edad;
    }

    fn actualizar_direccion(&mut self, dir: Option<String>) {
        if let Some(_) = &self.direccion{
            self.direccion = dir;
        }
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let per: Persona = Persona::new("Fermin".to_string(),25,Some("La Rioja 668".to_string()));
        let per2: Persona = Persona::new("Fermin".to_string(),25,None);
        let res = per.to_string();
        let res2 = per2.to_string();
        assert_eq!(res,"Hola, me llamo Fermin, tengo 25 años y vivo en La Rioja 668");
        assert_eq!(res2,"Hola, me llamo Fermin, tengo 25 años y no tengo una direccion asignada");
    }

    #[test]
    fn test_obtener_edad() {
        let per: Persona = Persona::new("Fermin".to_string(),25,Some("La Rioja 668".to_string()));
        let resultado = per.obtener_edad();
        assert_eq!(resultado, 25);
    }

    #[test]
    fn test_actualizar_direccion(){
        let mut per2: Persona = Persona::new("Fermin".to_string(),25,None);
        let mut per: Persona = Persona::new("Fermin".to_string(),25,Some("La Rioja 668".to_string()));
        per.actualizar_direccion(Some("Calle 4 entre 15 y 16".to_string()));
        per2.actualizar_direccion(None);
        let resultado = per.to_string();
        assert_eq!(resultado, "Hola, me llamo Fermin, tengo 25 años y vivo en Calle 4 entre 15 y 16");
        assert_eq!(per2.direccion, None);
    }
}    

