//
// Created by 16182 on 2/21/2024.
//
#include<vector>
#include<complex>
#include<immintrin.h>
#include<chrono>
#include<iostream>
#include<cstdlib>
#include<functional>

using cf = std::complex<float>;


static void sosfilt_fast(const float sos[3][6], cf* x, size_t len, cf zi[3][2]) {
    // iterate over every i sample section
    for (size_t i = 0; i < len; ++i)
    {
        cf x_c_0 = x[i];
        // iterate over every j section sample
        const float* section1 = sos[0];
        const float* section2 = sos[1];
        const float* section3 = sos[2];

        cf* zi_0 = zi[0];
        cf* zi_1 = zi[1];
        cf* zi_2 = zi[2];
        cf cp_zi_0[2];
        cf cp_zi_1[2];
        cf cp_zi_2[2];

        cf x_0  = section1[0] * x_c_0 + zi_0[0];
        cf x_1  = section2[0] * x_0 + zi_1[0];
        cf x_2  = section3[0] * x_1 + zi_2[0];

        cp_zi_0[0] = section1[1] * x_c_0 - section1[4] * x_0 + zi_0[1];
        cp_zi_1[0] = section2[1] * x_0 - section2[4] * x_1 + zi_1[1];
        cp_zi_2[0] = section3[1] * x_1 - section3[4] * x_2 + zi_2[1];

        cp_zi_0[1] = section1[2] * x_c_0 - section1[5] * x_0;
        cp_zi_1[1] = section2[2] * x_0 - section2[5] * x_1;
        cp_zi_2[1] = section3[2] * x_1 - section3[5] * x_2;

        x[i] = x_2;

        zi[0][0] = cp_zi_0[0];
        zi[1][0] = cp_zi_1[0];
        zi[2][0] = cp_zi_2[0];

        zi[0][1] = cp_zi_0[1];
        zi[1][1] = cp_zi_1[1];
        zi[2][1] = cp_zi_2[1];
    }
}



static void sosfilt_simd(const float sos[3][6], cf* x, size_t len, cf zi[3][2]) {
    // iterate over every i sample section
//    cf zix_0_temp[4] = {zi[0][0],zi[1][0],zi[2][0],0.0};
//    cf zix_1_temp[4] = {zi[0][1],zi[1][1],zi[2][1],0.0};
//    __m256 zix_0 = _mm256_loadu_ps((float*)zix_0_temp);
//    __m256 zix_1 = _mm256_loadu_ps((float*)zix_1_temp);
//    float sos0_0 = sos[0][0];
//    float sos1_0 = sos[1][0];
//    float sos2_0 = sos[2][0];
//    float sos21_0 = sos[2][0] * sos1_0;
//    float sos10_0 = sos[1][0] * sos0_0;
//    float sos210_0 = sos[2][0] * sos10_0;
//    __m256 vsos012_0 = _mm256_set_ps(sos0_0,sos0_0,sos10_0,sos10_0,sos210_0,sos210_0,1.0,1.0);
//    __m256 vsos12_0 = _mm256_set_ps(0.0,0.0,sos1_0,sos1_0,sos21_0,sos21_0,0.0,0.0);
//    __m256 vsos2_0 = _mm256_set_ps(0.0,0.0,0.0,0.0,sos2_0,sos2_0,0.0,0.0);
//    __m256 vsosx_1 = _mm256_set_ps(sos[0][1],sos[0][1],sos[1][1],sos[1][1],sos[2][1],sos[2][1],0.0,0.0);
//    __m256 vsosx_2 = _mm256_set_ps(sos[0][2],sos[0][2],sos[1][2],sos[1][2],sos[2][2],sos[2][2],0.0,0.0);
//    __m256 minus_vsosx_4 = _mm256_set_ps(-sos[0][4],-sos[0][4],-sos[1][4],-sos[1][4],-sos[2][4],-sos[2][4],0.0,0.0);
//    __m256 minus_vsosx_5 = _mm256_set_ps(-sos[0][5],-sos[0][5],-sos[1][5],-sos[1][5],-sos[2][5],-sos[2][5],0.0,0.0);
    if((intptr_t)x % 32 != 0){
        std::cerr << "Align issue\n";
        return;
    }
    __m128 s0_0 = _mm_set_ps1(sos[0][0]);
    __m128 s1_0 = _mm_set_ps1(sos[1][0]);
    __m128 s2_0 = _mm_set_ps1(sos[2][0]);
    __m128 s01_1 = _mm_set_ps(sos[0][1],sos[0][1],sos[1][1],sos[1][1]);
    __m128 s2x_1 = _mm_set_ps(sos[2][1],sos[2][1],0.0,0.0);
    __m128 s01_2 = _mm_set_ps(sos[0][2],sos[0][2],sos[1][2],sos[1][2]);
    __m128 s2x_2 = _mm_set_ps(sos[2][2],sos[2][2],0.0,0.0);
    __m128 s01_4 = _mm_set_ps(sos[0][4],sos[0][4],sos[1][4],sos[1][4]);
    __m128 s2x_4 = _mm_set_ps(sos[2][4],sos[2][4],0.0,0.0);
    __m128 s01_5 = _mm_set_ps(sos[0][5],sos[0][5],sos[1][5],sos[1][5]);
    __m128 s2x_5 = _mm_set_ps(sos[2][5],sos[2][5],0.0,0.0);
    __m128 zi01_0 = _mm_set_ps(zi[0][0].real(),zi[0][0].imag(),zi[1][0].real(),zi[1][0].imag());
    __m128 zi2x_0 = _mm_set_ps(zi[2][0].real(),zi[2][0].imag(),0.0,0.0);
    __m128 zi01_1 = _mm_set_ps(zi[0][1].real(),zi[0][1].imag(),zi[0][1].real(),zi[0][1].imag());
    __m128 zi2x_1 = _mm_set_ps(zi[2][1].real(),zi[2][1].imag(),0.0,0.0);

    for (size_t i = 0; i < len; i += 2)
    {
        __m128 zi10_0 = _mm_permutevar_ps(zi01_0, _mm_set_epi32(2,3,0,1));
        __m128 x_c_01 = _mm_load_ps((float*)&x[i]);
        {
            __m128 x_0 = _mm_fmadd_ss(s0_0, x_c_01, zi01_0);  //[x_0, -]
            __m128 x_1 = _mm_fmadd_ss(s1_0, x_0, zi10_0);     //[x_1, -]
            __m128 x_2 = _mm_fmadd_ss(s2_0, x_1, zi2x_0);     //[x_2, -]

            __m128 tmp0 = _mm_fmadd_ss(s01_4, x_0, zi01_1);
            __m128 tmp1 = _mm_fmadd_ss(s2x_4, x_2, zi2x_1);

            zi01_0 = _mm_fmadd_ss(s01_1, x_c_01, tmp0);
            zi2x_0 = _mm_fmadd_ss(s2x_1, x_c_01, tmp1);

            __m128 tmp2 = _mm_mul_ss(s01_5, x_0);
            __m128 tmp3 = _mm_mul_ss(s2x_5, x_2);

            zi01_1 = _mm_fmsub_ss(s01_2, x_c_01, tmp2);
            zi2x_1 = _mm_fmsub_ss(s2x_2, x_c_01, tmp3);
        }
        {
            __m128 x_0 = _mm_fmadd_ss(s0_0, x_c_01, zi01_0);  //[x_0, -]
            __m128 x_1 = _mm_fmadd_ss(s1_0, x_0, zi10_0);     //[x_1, -]
            __m128 x_2 = _mm_fmadd_ss(s2_0, x_1, zi2x_0);     //[x_2, -]

            __m128 tmp0 = _mm_fmadd_ss(s01_4, x_0, zi01_1);
            __m128 tmp1 = _mm_fmadd_ss(s2x_4, x_2, zi2x_1);

            zi01_0 = _mm_fmadd_ss(s01_1, x_c_01, tmp0);
            zi2x_0 = _mm_fmadd_ss(s2x_1, x_c_01, tmp1);

            __m128 tmp2 = _mm_mul_ss(s01_5, x_0);
            __m128 tmp3 = _mm_mul_ss(s2x_5, x_2);

            zi01_1 = _mm_fmsub_ss(s01_2, x_c_01, tmp2);
            zi2x_1 = _mm_fmsub_ss(s2x_2, x_c_01, tmp3);


            _mm_store_ps((float*)&x[i], x_2);
        }

//        cp_zi_0[0] = section0[1] * x_c_0  - section0[4] * x_0 + zi_0[1];
//        cp_zi_1[0] = section1[1] * x_0    - section1[4] * x_1 + zi_1[1];
//        cp_zi_2[0] = section2[1] * x_1    - section2[4] * x_2 + zi_2[1];

//        cp_zi_0[1] = section0[2] * x_c_0  - section0[5] * x_0;
//        cp_zi_1[1] = section1[2] * x_0    - section1[5] * x_1;
//        cp_zi_2[1] = section2[2] * x_1    - section2[5] * x_2;

//        cf x_0  = section0[0] * x_c_0 + zi_0[0];
//        cf x_1  = section1[0] * x_0 + zi_1[0];
//        cf x_2  = section2[0] * x_1 + zi_2[0];

//        cp_zi_0[0] = section0[1] * x_c_0 - section0[4] * x_0 + zi_0[1];
//        cp_zi_1[0] = section1[1] * x_0 - section1[4] * x_1 + zi_1[1];

//        cp_zi_2[0] = section2[1] * x_1 - section2[4] * x_2 + zi_2[1];
//        cp_zi_0[1] = section0[2] * x_c_0 - section0[5] * x_0;

//        cp_zi_1[1] = section1[2] * x_0 - section1[5] * x_1;
//        cp_zi_2[1] = section2[2] * x_1 - section2[5] * x_2;
////        __m256 v1, v2, vx, vx2, v3, v4, x_i;
////
//////        x_i = _mm256_permutevar8x32_ps(x_0, _mm256_set_epi32(0,1,0,1,0,1,0,1));
////        x_i = x_0;
//////        v1 = _mm256_fmadd_ps(vsos2_0, _mm256_permutevar8x32_ps(zix_0, _mm256_set_epi32(6,6,6,6,2,3,6,6)), zix_0);
//////        v2 = _mm256_fmadd_ps(vsos12_0, _mm256_permutevar8x32_ps(zix_0, _mm256_set_epi32(6,6,0,1,0,1,6,6)), v1);
//        __m256 v1 = _mm256_fmadd_ps(vsos2_0, zix_0, zix_0);
//        __m256 v2 = _mm256_fmadd_ps(vsos12_0, zix_0, v1);
//        __m256 vx = _mm256_fmadd_ps(vsos012_0, x_0, v2); //[x_0, x_1, x_2, x_i]
//
//        __m256 vx2 = _mm256_permutevar8x32_ps(vx, _mm256_set_epi32(6,7,0,1,2,3,4,5)); //[x_i, x_1, x_2, -]
//        __m256 v3 = _mm256_fmadd_ps(vx, minus_vsosx_4, zix_1);
//        zix_0 = _mm256_fmadd_ps(vsosx_1, vx2, v3);
//        __m256 v4 = _mm256_mul_ps(minus_vsosx_5, vx);
//        zix_1 = _mm256_fmadd_ps(vsosx_2, vx2, v4);
//
//        _mm256_store_ps((float*)&x[i], vx);
    }
}


static constexpr float LOW_PASS_SOS_33K[3][6] = {
    {3.869494405731452e-12,7.738988811462904e-12,3.869494405731452e-12,1.0,-0.989582475318754,0.0,},
    {1.0,2.0,1.0,1.0,-1.98308989599488,0.9831986360344092,},
    {1.0,1.0,0.0,1.0,-1.9934396492520414,0.9935489568062257,},
};

void time(std::function<void(void)> const& f){
    auto t1 = std::chrono::steady_clock::now();
    f();
    auto t2 = std::chrono::steady_clock::now();
    std::cout << ((t2-t1)/std::chrono::microseconds(1)/1000.0) << std::endl;
}

int main(){
    std::vector<cf> samps;
    for(int i = 0; i< 1024*16; i++){
        samps.push_back(cf(std::sin(i/100.f),std::cos(i/100.f)));
    }
    cf* s1 = (cf*)_aligned_malloc(samps.size() * sizeof(cf), 32);
    cf* s2 = (cf*)_aligned_malloc(samps.size() * sizeof(cf), 32);

    cf z1[3][2]{};
    cf z2[3][2]{};
    time([&](){for(int i = 0; i < 1024; i++) sosfilt_fast(LOW_PASS_SOS_33K, s1, samps.size(), z1);});
    time([&](){for(int i = 0; i < 1024; i++) sosfilt_simd(LOW_PASS_SOS_33K, s2, samps.size(), z2);});
}