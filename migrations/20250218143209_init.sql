-- Add migration script here



CREATE TABLE IF NOT EXISTS public.t_users (
                                              id SERIAL PRIMARY KEY,
                                              username TEXT NOT NULL,
                                              name TEXT NOT NULL,
                                              email TEXT NOT NULL UNIQUE,
                                              password TEXT NOT NULL,
                                              phone TEXT NOT NULL,
                                              admin BOOLEAN NOT NULL DEFAULT FALSE,
                                              address TEXT NOT NULL,
                                              secret_otp TEXT NULL,
                                              created_by TEXT NULL,
                                              created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                                              updated_by TEXT NULL,
                                              updated_at TIMESTAMP WITH TIME ZONE NULL,
                                              last_auth_change TIMESTAMP WITH TIME ZONE NULL
);



CREATE TABLE IF NOT EXISTS public.t_tests (
                                              id SERIAL PRIMARY KEY,
                                              name TEXT NOT NULL,
                                              year TEXT NOT NULL,
                                              total_parts INT NOT NULL, -- Total number of parts in the exam (e.g., 7 for TOEIC)
                                              questions INT NOT NULL, -- Total number of questions in the exam
                                              is_active BOOLEAN NOT NULL DEFAULT TRUE,
                                              test_type TEXT NOT NULL, -- Type of exam (e.g., "TOEIC", "IELTS")
                                              created_by TEXT NULL, -- User who created the exam
                                              created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
                                              updated_by TEXT NULL,
                                              updated_at TIMESTAMP WITH TIME ZONE NULL
);



CREATE TABLE IF NOT EXISTS public.t_test_parts (
                                                   id SERIAL PRIMARY KEY,
                                                   part TEXT NOT NULL, -- Part number (e.g., "Part 1", "Part 2", "Part 3", etc.)
                                                   question_number INT NOT NULL, -- The question number within the exam
                                                   paragraph TEXT NULL, -- The paragraph or conversation for the question (used in Parts 3, 4, 6, 7)
                                                   question TEXT NOT NULL, -- The individual question
                                                   options JSONB NOT NULL, -- JSON array for multiple-choice answers
                                                   correct_answer TEXT NOT NULL CHECK (correct_answer IN ('A', 'B', 'C', 'D')), -- Ensures only A, B, C, or D
    audio_url TEXT NULL, -- URL for the audio file (only for Listening sections like Part 1-4)
    image_url TEXT NULL, -- URL for the image (if applicable, e.g., Part 1)
    explanation TEXT NULL, -- Explanation of the correct answer
    test_section TEXT NOT NULL CHECK (test_section IN ('Listening', 'Reading')), -- Indicates whether the question is for Listening or Readingcreated_by TEXT NULL, -- User who created the question
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_by TEXT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NULL
                             );



CREATE TABLE IF NOT EXISTS public.t_otp (
                                            id SERIAL PRIMARY KEY,
                                            phone_number TEXT NOT NULL, -- Phone number to which the OTP is sent
                                            authen_key TEXT NOT NULL, -- Unique key to identify the authentication request
                                            otp_code TEXT NOT NULL, -- The OTP code (consider hashing for security)
                                            expires_at TIMESTAMP WITH TIME ZONE NOT NULL, -- Expiration time for the OTP
                                            is_verified BOOLEAN DEFAULT FALSE, -- Indicates if the OTP has been used/verified
                                            created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP, -- Time of OTP generation
                                            updated_at TIMESTAMP WITH TIME ZONE NULL -- Time of last update (if applicable)
);


