use std::{
    fs::File,
    io::{BufWriter, Write},
};

use neighbors::Particle as MethodParticle;

use crate::{constants::SIMULATION_LENGHT, particle::Particle, Result};

const CORNERS: [(f64, f64); 4] = [
    (0.0, 0.0),
    (SIMULATION_LENGHT, 0.0),
    (0.0, SIMULATION_LENGHT),
    (SIMULATION_LENGHT, SIMULATION_LENGHT),
];

pub fn output_simulation(file: &File, particles: &Vec<Particle>) -> Result<()> {
    let mut writer = BufWriter::new(file);

    let particle_count = particles.len() + CORNERS.len();
    writeln!(writer, "{particle_count}")?;
    writeln!(writer, "Properties=pos:R:2:velo:R:2:radius:R:1 pbc=\"F F\"",)?;

    // NOTE: Write the particles
    for particle in particles {
        let coordinates = particle.get_coordinates();
        let velocities = particle.get_velocities();

        writeln!(
            writer,
            "{:.12} {:.12} {:.12} {:.12} {:.4}",
            coordinates.0,
            coordinates.1,
            velocities.0,
            velocities.1,
            particle.get_radius(),
        )?;
    }

    // NOTE: Write the corners
    for corner in &CORNERS {
        writeln!(writer, "{:.12} {:.12} 0 0 0.001", corner.0, corner.1)?;
    }

    Ok(())
}