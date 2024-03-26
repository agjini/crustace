pub struct Univers {
    pub solar_system: Vec<Vec<Cellule>>
}

#[derive(Debug)]
pub struct Cellule {
    pub x :usize,
    pub y: usize,
    pub alive: bool
}

impl Cellule {
    pub fn find_voisin (univer:&Univers,x:usize,y:usize) -> Vec<&Cellule>{
        let mut mes_voisins:Vec<&Cellule> = Vec::new();
        //gauche
        if !univer.out_min(y){
            mes_voisins.push(&univer.solar_system[x][y-1]);
        }

        //droite
        if !univer.out_max(y) {
            mes_voisins.push(&univer.solar_system[x][y+1])
        }

        //haut
        if !univer.out_min(x) {
            mes_voisins.push(&univer.solar_system[x-1][y])
        }

        //haut gauche
        if !univer.out_min(x) && !univer.out_min(y){
            mes_voisins.push(&univer.solar_system[x-1][y-1])
        }

        //haut droite
        if !univer.out_min(x) && !univer.out_max(y){
            mes_voisins.push(&univer.solar_system[x-1][y+1])
        }

        //bas
        if !univer.out_max(x) {
            mes_voisins.push(&univer.solar_system[x+1][y])
        }

        //bas gauche
        if !univer.out_max(x) && !univer.out_min(y) {
            mes_voisins.push(&univer.solar_system[x+1][y-1])
        }

        //bas droite
        if !univer.out_max(x) && !univer.out_max(y) {
            mes_voisins.push(&univer.solar_system[x+1][y+1])
        }

        return mes_voisins;
    }
}

impl Univers {
    pub fn out_min (&self, value:usize) -> bool {
        usize::overflowing_sub(value, 1).1
    }

    pub fn out_max(&self, value:usize) -> bool {
        (value + 1) > self.solar_system.len()
    }
}

fn main() {

}



#[test]
fn create_univers_with_size(){
//je créer un univers de taille défini j'obtiens univers de cette taille
    let univers_5_5 = Univers {
        solar_system: vec![
            vec![Cellule{x:0,y:0,alive:true}, Cellule{x:0,y:1,alive:false}, Cellule{x:0,y:2,alive:true}, Cellule{x:0,y:3,alive:true}, Cellule{x:0,y:4,alive:true}],
            vec![Cellule{x:1,y:0,alive:false}, Cellule{x:1,y:1,alive:true}, Cellule{x:1,y:2,alive:true}, Cellule{x:1,y:3,alive:true}, Cellule{x:1,y:4,alive:true}],
            vec![Cellule{x:2,y:0,alive:false}, Cellule{x:2,y:1,alive:true}, Cellule{x:2,y:2,alive:true}, Cellule{x:2,y:3,alive:true}, Cellule{x:2,y:4,alive:true}],
            vec![Cellule{x:3,y:0,alive:false}, Cellule{x:3,y:1,alive:true}, Cellule{x:3,y:2,alive:true}, Cellule{x:3,y:3,alive:true}, Cellule{x:3,y:4,alive:true}],
            vec![Cellule{x:4,y:0,alive:false}, Cellule{x:4,y:1,alive:true}, Cellule{x:4,y:2,alive:true}, Cellule{x:4,y:3,alive:true}, Cellule{x:4,y:4,alive:true}],
        ]
    };

    //nb ligne
    assert_eq!(univers_5_5.solar_system.len(), 5);
    //nb col dans chaque ligne
    for  ligne in univers_5_5.solar_system {
        assert_eq!(ligne.len(), 5)
    }
}

#[test]
fn test_voisin(){
    let univers_5_5 = Univers {
        solar_system: vec![
            vec![Cellule{x:0,y:0,alive:true}, Cellule{x:0,y:1,alive:false}, Cellule{x:0,y:2,alive:true}, Cellule{x:0,y:3,alive:false}, Cellule{x:0,y:4,alive:true}],
            vec![Cellule{x:1,y:0,alive:false}, Cellule{x:1,y:1,alive:false}, Cellule{x:1,y:2,alive:true}, Cellule{x:1,y:3,alive:false}, Cellule{x:1,y:4,alive:true}],
            vec![Cellule{x:2,y:0,alive:false}, Cellule{x:2,y:1,alive:true}, Cellule{x:2,y:2,alive:true}, Cellule{x:2,y:3,alive:true}, Cellule{x:2,y:4,alive:true}],
            vec![Cellule{x:3,y:0,alive:false}, Cellule{x:3,y:1,alive:true}, Cellule{x:3,y:2,alive:true}, Cellule{x:3,y:3,alive:true}, Cellule{x:3,y:4,alive:true}],
            vec![Cellule{x:4,y:0,alive:false}, Cellule{x:4,y:1,alive:true}, Cellule{x:4,y:2,alive:true}, Cellule{x:4,y:3,alive:true}, Cellule{x:4,y:4,alive:true}],
        ]
    };

    let nb_voisin:Vec<&Cellule> = Cellule::find_voisin(&univers_5_5, 0, 2);
    // nb voisin en vie ou mort
    assert_eq!(nb_voisin.len(), 5);
}

#[test]
fn test_voisin_bordure() {
    let univers_5_5 = Univers {
        solar_system: vec![
            vec![Cellule{x:0,y:0,alive:true}, Cellule{x:0,y:1,alive:false}, Cellule{x:0,y:2,alive:true}, Cellule{x:0,y:3,alive:true}, Cellule{x:0,y:4,alive:true}],
            vec![Cellule{x:1,y:0,alive:false}, Cellule{x:1,y:1,alive:true}, Cellule{x:1,y:2,alive:true}, Cellule{x:1,y:3,alive:true}, Cellule{x:1,y:4,alive:true}],
            vec![Cellule{x:2,y:0,alive:false}, Cellule{x:2,y:1,alive:true}, Cellule{x:2,y:2,alive:true}, Cellule{x:2,y:3,alive:true}, Cellule{x:2,y:4,alive:true}],
            vec![Cellule{x:3,y:0,alive:false}, Cellule{x:3,y:1,alive:true}, Cellule{x:3,y:2,alive:true}, Cellule{x:3,y:3,alive:true}, Cellule{x:3,y:4,alive:true}],
            vec![Cellule{x:4,y:0,alive:false}, Cellule{x:4,y:1,alive:true}, Cellule{x:4,y:2,alive:true}, Cellule{x:4,y:3,alive:true}, Cellule{x:4,y:4,alive:true}],
        ]
    };

    let nb_voisin: Vec<&Cellule> = Cellule::find_voisin(&univers_5_5,0, 0);
    //nb voisin en vie ou mort
    assert_eq!(nb_voisin.len(), 3);
}

#[test]
fn test_voisin_is_alive() {
    //data
    let univers_5_5 = Univers {
        solar_system: vec![
            vec![Cellule{x:0,y:0,alive:true}, Cellule{x:0,y:1,alive:false}, Cellule{x:0,y:2,alive:true}, Cellule{x:0,y:3,alive:true}, Cellule{x:0,y:4,alive:true}],
            vec![Cellule{x:1,y:0,alive:false}, Cellule{x:1,y:1,alive:true}, Cellule{x:1,y:2,alive:false}, Cellule{x:1,y:3,alive:true}, Cellule{x:1,y:4,alive:true}],
            vec![Cellule{x:2,y:0,alive:false}, Cellule{x:2,y:1,alive:false}, Cellule{x:2,y:2,alive:true}, Cellule{x:2,y:3,alive:false}, Cellule{x:2,y:4,alive:true}],
            vec![Cellule{x:3,y:0,alive:false}, Cellule{x:3,y:1,alive:true}, Cellule{x:3,y:2,alive:false}, Cellule{x:3,y:3,alive:true}, Cellule{x:3,y:4,alive:true}],
            vec![Cellule{x:4,y:0,alive:false}, Cellule{x:4,y:1,alive:true}, Cellule{x:4,y:2,alive:true}, Cellule{x:4,y:3,alive:true}, Cellule{x:4,y:4,alive:true}],
        ]
    };
    let nb_voisin: Vec<&Cellule> = Cellule::find_voisin(&univers_5_5,2, 2);

    //action
    let mut count: usize = 0;
    for cellule in nb_voisin{
        if cellule.alive {
            count += 1;
        }
    }
    //nb voisin en vie
    assert_eq!(count, 4);
}