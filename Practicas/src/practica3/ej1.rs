struct Persona{
    nombre: String,
    edad: i32,
    direccion: String 
}

impl Persona{
    fn new(name:String, age: i32, direc: String) -> Persona{
      Persona {
        nombre: name.to_string(),
        edad: age,
        direccion: direc.to_string(),
      }  
    }

    fn to_string(&self) -> String{
        format!(
            "Hola, me llamo {}, tengo {} años y vivo en {}", self.nombre, self.edad, self.direccion
        )
    }

    fn obtener_edad(self) -> i32 {
        return self.edad;
    }

    fn actualizar_direccion(&mut self, dir: String) {
        self.direccion = dir.to_string();
    }    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_string() {
        let per: Persona = Persona::new("Fermin".to_string(),25,"La Rioja 668".to_string());
        let res = per.to_string();
        assert_eq!(res,"Hola, me llamo Fermin, tengo 25 años y vivo en La Rioja 668");
    }
}    

