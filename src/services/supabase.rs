use postgrest::Postgrest;

pub async fn create_client() -> Postgrest {
    let api_key = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InRkZ29kYWdqZmdobmhvcGZwY3Z5Iiwicm9sZSI6ImFub24iLCJpYXQiOjE2NjczMjE2NjYsImV4cCI6MTk4Mjg5NzY2Nn0.v9gylG-7wf3cQfFNu7d9qA2LMeE6ww8FJsAMBu9PSaU";
    let url = "https://tdgodagjfghnhopfpcvy.supabase.co/rest/v1";
    let client = Postgrest::new(url)
        .insert_header("apikey", api_key)
        .insert_header("Access-Control-Allow-Origin", "*")
        .insert_header("Access-Control-Allow-Headers", "*")
        .insert_header("Access-Control-Allow-Credentials", "true")
        .insert_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
        .insert_header("Sec-Fetch-Site", "none")
        ;
    return client
}