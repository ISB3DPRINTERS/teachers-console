import { createClient } from '@supabase/supabase-js'
const 
const supabase = createClient(PUBLIC_SUPABASE_URL, PUBLIC_SUPABASE_ANON_KEY)

export default supabase