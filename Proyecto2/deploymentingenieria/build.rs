fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Compila el archivo .proto
    tonic_build::compile_protos("proto/student_service.proto")?;
    

    println!("cargo:rerun-if-changed=proto/student_service.proto");
    Ok(())
    // Indicar a Cargo que vuelva a ejecutar el script si el archivo .proto cambia
    //println!("cargo:rerun-if-changed={}", proto_file);
}